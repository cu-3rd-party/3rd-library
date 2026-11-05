//! PostgreSQL для интеграционных тестов.
//!
//! На тестовый бинарь поднимается **один** сервер:
//! - если задан `TEST_DATABASE_URL` — используется он (CI: `services: postgres`, Docker не нужен);
//! - иначе — один контейнер через Testcontainers, удаляется при выходе процесса.
//!
//! В сервере один раз создаётся БД-шаблон с применёнными миграциями, а каждый тест получает
//! свою копию (`CREATE DATABASE ... TEMPLATE`, миллисекунды) — изоляция как с контейнером на тест,
//! но без его запуска.

use std::sync::{Mutex, OnceLock, mpsc};
use std::thread;
use std::time::Duration;

use library_migration::MIGRATOR;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use testcontainers_modules::postgres::Postgres;
use testcontainers_modules::testcontainers::runners::AsyncRunner;
use testcontainers_modules::testcontainers::{ContainerAsync, ImageExt};
use url::Url;
use uuid::Uuid;

/// Внешний сервер для тестов, например `postgres://postgres:postgres@postgres:5432/postgres`.
pub const DATABASE_URL_ENV: &str = "TEST_DATABASE_URL";

/// Как в проде (docker-compose).
const POSTGRES_TAG: &str = "16-alpine";

/// Отдельная мигрированная пустая БД для одного теста.
pub struct TestDatabase {
    pub name: String,
    pub url: String,
}

impl TestDatabase {
    pub async fn create() -> Self {
        let server = server();
        let name = format!("test_{}", Uuid::now_v7().simple());

        let mut admin = PgConnection::connect(server.admin_url.as_str())
            .await
            .expect("connect to test PostgreSQL");
        create_database(&mut admin, &name, Some(&server.template))
            .await
            .expect("create test database");
        admin.close().await.ok();

        created_databases().lock().unwrap().push(name.clone());
        Self {
            url: database_url(&server.admin_url, &name),
            name,
        }
    }

    pub async fn connect(&self) -> PgPool {
        PgPoolOptions::new()
            .max_connections(5)
            .connect(&self.url)
            .await
            .expect("connect to test database")
    }
}

struct Server {
    /// URL служебной БД: через неё создаются тестовые БД.
    admin_url: Url,
    /// Имя БД-шаблона с применёнными миграциями.
    template: String,
}

/// Сигнал фоновому потоку «пора убирать» и ответ «убрано».
type Shutdown = (tokio::sync::oneshot::Sender<()>, mpsc::Receiver<()>);

static SERVER: OnceLock<Result<Server, String>> = OnceLock::new();
static SHUTDOWN: Mutex<Option<Shutdown>> = Mutex::new(None);
static CREATED: OnceLock<Mutex<Vec<String>>> = OnceLock::new();

fn created_databases() -> &'static Mutex<Vec<String>> {
    CREATED.get_or_init(Default::default)
}

fn server() -> &'static Server {
    SERVER
        .get_or_init(start)
        .as_ref()
        .unwrap_or_else(|error| panic!("{error}"))
}

/// Сервер живёт в отдельном потоке со своим runtime: у каждого `#[tokio::test]` runtime свой
/// и умирает вместе с тестом, а контейнер и его Docker-клиент должны пережить все тесты.
fn start() -> Result<Server, String> {
    let (ready_tx, ready_rx) = mpsc::channel();
    let (stop_tx, stop_rx) = tokio::sync::oneshot::channel();
    let (done_tx, done_rx) = mpsc::channel();

    thread::Builder::new()
        .name("test-postgres".into())
        .spawn(move || {
            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("build test-postgres runtime");
            runtime.block_on(async move {
                let container = match boot().await {
                    Ok((server, container)) => {
                        ready_tx.send(Ok(server)).ok();
                        container
                    }
                    Err(error) => {
                        ready_tx.send(Err(error)).ok();
                        return;
                    }
                };

                stop_rx.await.ok();
                match container {
                    Some(container) => {
                        container.rm().await.ok();
                    }
                    None => drop_created_databases().await,
                }
                done_tx.send(()).ok();
            });
        })
        .map_err(|error| error.to_string())?;

    let server = ready_rx
        .recv()
        .map_err(|_| "test-postgres thread panicked".to_owned())??;
    *SHUTDOWN.lock().unwrap() = Some((stop_tx, done_rx));
    Ok(server)
}

async fn boot() -> Result<(Server, Option<ContainerAsync<Postgres>>), String> {
    let (admin_url, container) = match std::env::var(DATABASE_URL_ENV) {
        Ok(url) => (
            Url::parse(&url).map_err(|error| format!("invalid {DATABASE_URL_ENV}: {error}"))?,
            None,
        ),
        Err(_) => {
            let container = Postgres::default()
                .with_tag(POSTGRES_TAG)
                .start()
                .await
                .map_err(|error| format!("failed to start PostgreSQL container (is Docker running?): {error}"))?;
            let host = container.get_host().await.map_err(|error| error.to_string())?;
            let port = container
                .get_host_port_ipv4(5432)
                .await
                .map_err(|error| error.to_string())?;
            let url = format!("postgres://postgres:postgres@{host}:{port}/postgres");
            (Url::parse(&url).map_err(|error| error.to_string())?, Some(container))
        }
    };

    let template = format!("template_{}", Uuid::now_v7().simple());
    create_template(&admin_url, &template)
        .await
        .map_err(|error| format!("failed to prepare template database: {error}"))?;
    created_databases().lock().unwrap().push(template.clone());

    Ok((Server { admin_url, template }, container))
}

async fn create_template(admin_url: &Url, template: &str) -> Result<(), sqlx::Error> {
    let mut admin = PgConnection::connect(admin_url.as_str()).await?;
    create_database(&mut admin, template, None).await?;
    admin.close().await?;

    // все соединения с шаблоном должны быть закрыты, иначе CREATE DATABASE ... TEMPLATE упадёт
    let mut db = PgConnection::connect(&database_url(admin_url, template)).await?;
    MIGRATOR.run(&mut db).await?;
    db.close().await
}

async fn create_database(admin: &mut PgConnection, name: &str, template: Option<&str>) -> Result<(), sqlx::Error> {
    let sql = match template {
        Some(template) => format!(r#"CREATE DATABASE "{name}" TEMPLATE "{template}""#),
        None => format!(r#"CREATE DATABASE "{name}""#),
    };
    // параллельные копии одного шаблона иногда конфликтуют ("source database is being accessed")
    let mut attempt = 0;
    loop {
        match admin.execute(sql.as_str()).await {
            Ok(_) => return Ok(()),
            Err(error) if attempt < 20 && error.to_string().contains("is being accessed by other users") => {
                attempt += 1;
                tokio::time::sleep(Duration::from_millis(50)).await;
            }
            Err(error) => return Err(error),
        }
    }
}

/// Только для внешнего сервера: контейнер удаляется целиком.
async fn drop_created_databases() {
    let Some(server) = SERVER.get().and_then(|server| server.as_ref().ok()) else {
        return;
    };
    let Ok(mut admin) = PgConnection::connect(server.admin_url.as_str()).await else {
        return;
    };
    let names = std::mem::take(&mut *created_databases().lock().unwrap());
    for name in names {
        admin
            .execute(format!(r#"DROP DATABASE IF EXISTS "{name}" WITH (FORCE)"#).as_str())
            .await
            .ok();
    }
    admin.close().await.ok();
}

fn database_url(admin_url: &Url, name: &str) -> String {
    let mut url = admin_url.clone();
    url.set_path(name);
    url.into()
}

/// Выход из тестового бинаря: удалить контейнер (или созданные БД на внешнем сервере).
#[dtor::dtor(unsafe)]
fn shutdown() {
    let Some((stop, done)) = SHUTDOWN.lock().ok().and_then(|mut guard| guard.take()) else {
        return;
    };
    if stop.send(()).is_ok() {
        done.recv_timeout(Duration::from_secs(15)).ok();
    }
}
