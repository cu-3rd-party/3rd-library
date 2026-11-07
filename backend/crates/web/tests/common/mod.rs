//! Приложение для интеграционных тестов: настоящая БД на тест (и сессии в ней), вместо S3/SMTP — фейки в памяти.
#![allow(dead_code)]

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use axum::Router;
use axum::body::Body;
use axum::http::{HeaderMap, Method, Request, StatusCode, header};
use bytes::Bytes;
use http_body_util::BodyExt;
use library_application::ports::{ExternalResult, FileStorage, Mailer};
use library_infrastructure::persistence::Store;
use library_infrastructure::{Adapters, Infrastructure};
use library_test_support::TestDatabase;
use library_web::{AppState, app, cors_layer};
use serde_json::{Value, json};
use sqlx::PgPool;
use tower::ServiceExt;
use uuid::Uuid;

#[derive(Default)]
pub struct MemoryStorage(pub Mutex<HashMap<String, Bytes>>);

#[async_trait]
impl FileStorage for MemoryStorage {
    async fn put(&self, key: &str, content: Bytes, _content_type: &str) -> ExternalResult<()> {
        self.0.lock().unwrap().insert(key.to_owned(), content);
        Ok(())
    }
}

/// Запоминает последние коды подтверждения по адресам.
#[derive(Default)]
pub struct MemoryMailer(Mutex<HashMap<String, String>>);

impl Mailer for MemoryMailer {
    fn send_verification_code(&self, email: &str, code: &str) {
        self.0.lock().unwrap().insert(email.to_owned(), code.to_owned());
    }
}

pub struct Response {
    pub status: StatusCode,
    pub headers: HeaderMap,
    pub body: Value,
}

/// Часть multipart-запроса: `(имя, имя файла, тип, содержимое)`.
pub type Part<'a> = (&'a str, Option<&'a str>, Option<&'a str>, &'a [u8]);

pub struct TestApp {
    pub router: Router,
    pub db: PgPool,
    pub store: Arc<Store>,
    pub mailer: Arc<MemoryMailer>,
    pub storage: Arc<MemoryStorage>,
    _database: TestDatabase,
}

impl TestApp {
    pub async fn spawn() -> Self {
        // логи — в вывод упавшего теста
        tracing_subscriber::fmt()
            .with_test_writer()
            .with_env_filter("warn")
            .try_init()
            .ok();
        let database = TestDatabase::create().await;
        let db = database.connect().await;
        let mailer = Arc::new(MemoryMailer::default());
        let storage = Arc::new(MemoryStorage::default());
        let infrastructure = Infrastructure::new(
            db.clone(),
            "test-hmac-key",
            Adapters {
                storage: storage.clone(),
                mailer: mailer.clone(),
                rate_limiter: None,
            },
        )
        .await
        .unwrap();
        Self {
            store: infrastructure.store.clone(),
            router: app(AppState::new(infrastructure), cors_layer("").unwrap(), true),
            db,
            mailer,
            storage,
            _database: database,
        }
    }

    pub async fn send(&self, request: Request<Body>) -> Response {
        let response = self.router.clone().oneshot(request).await.unwrap();
        let status = response.status();
        let headers = response.headers().clone();
        let bytes = response.into_body().collect().await.unwrap().to_bytes();
        let body =
            serde_json::from_slice(&bytes).unwrap_or_else(|_| Value::String(String::from_utf8_lossy(&bytes).into()));
        Response { status, headers, body }
    }

    pub async fn json(&self, method: Method, uri: &str, token: Option<&str>, body: Option<Value>) -> Response {
        let mut request = Request::builder().method(method).uri(uri);
        if let Some(token) = token {
            request = request.header(header::AUTHORIZATION, format!("Token {token}"));
        }
        let body = match body {
            Some(body) => {
                request = request.header(header::CONTENT_TYPE, "application/json");
                Body::from(body.to_string())
            }
            None => Body::empty(),
        };
        self.send(request.body(body).unwrap()).await
    }

    pub async fn get(&self, uri: &str, token: Option<&str>) -> Response {
        self.json(Method::GET, uri, token, None).await
    }

    pub async fn post(&self, uri: &str, token: Option<&str>, body: Value) -> Response {
        self.json(Method::POST, uri, token, Some(body)).await
    }

    pub async fn multipart(&self, method: Method, uri: &str, token: &str, parts: &[Part<'_>]) -> Response {
        const BOUNDARY: &str = "test-boundary-7f3a";
        let mut body = Vec::new();
        for (name, file_name, content_type, content) in parts {
            body.extend_from_slice(
                format!("--{BOUNDARY}\r\nContent-Disposition: form-data; name=\"{name}\"").as_bytes(),
            );
            if let Some(file_name) = file_name {
                body.extend_from_slice(format!("; filename=\"{file_name}\"").as_bytes());
            }
            body.extend_from_slice(b"\r\n");
            if let Some(content_type) = content_type {
                body.extend_from_slice(format!("Content-Type: {content_type}\r\n").as_bytes());
            }
            body.extend_from_slice(b"\r\n");
            body.extend_from_slice(content);
            body.extend_from_slice(b"\r\n");
        }
        body.extend_from_slice(format!("--{BOUNDARY}--\r\n").as_bytes());

        let request = Request::builder()
            .method(method)
            .uri(uri)
            .header(header::AUTHORIZATION, format!("Token {token}"))
            .header(
                header::CONTENT_TYPE,
                format!("multipart/form-data; boundary={BOUNDARY}"),
            )
            .body(Body::from(body))
            .unwrap();
        self.send(request).await
    }

    pub fn code_for(&self, email: &str) -> String {
        self.mailer
            .0
            .lock()
            .unwrap()
            .get(email)
            .cloned()
            .expect("verification code was sent")
    }

    pub async fn register(&self, email: &str) -> Response {
        self.post(
            "/api/auth/register",
            None,
            json!({"name": format!("User {email}"), "email": email, "password": "password123"}),
        )
        .await
    }

    /// Регистрация и подтверждение почты; возвращает `(user_id, access_token, refresh_token)`.
    pub async fn verified_user(&self, email: &str) -> (Uuid, String, String) {
        assert_eq!(self.register(email).await.status, StatusCode::OK);
        let verified = self
            .post(
                "/api/auth/verify-email",
                None,
                json!({"email": email, "code": self.code_for(email)}),
            )
            .await;
        assert_eq!(verified.status, StatusCode::OK, "{}", verified.body);
        (
            verified.body["user"]["id"].as_str().unwrap().parse().unwrap(),
            verified.body["tokens"]["access_token"].as_str().unwrap().to_owned(),
            verified.body["tokens"]["refresh_token"].as_str().unwrap().to_owned(),
        )
    }

    /// Модератор: роль выдаётся напрямую в БД, как в проде; бэкенд подхватывает её при сверке с БД
    /// (раз в минуту, здесь — сразу).
    pub async fn moderator(&self, email: &str) -> String {
        let (user_id, token, _) = self.verified_user(email).await;
        sqlx::query("update web_user set roles = array['user', 'moderator'] where user_id = $1")
            .bind(user_id)
            .execute(&self.db)
            .await
            .unwrap();
        self.store.sync_auth().await.unwrap();
        token
    }

    /// Заявка с файлами от имени `token`; возвращает тело ответа.
    pub async fn submit(&self, token: &str, title: &str, files: &[&str]) -> Response {
        let data = json!({
            "title": title,
            "description": "Lecture notes",
            "courses": ["1"],
            "subjects": ["math"],
            "type": "notes",
            "difficulty": "easy"
        })
        .to_string();
        let mut parts: Vec<Part> = vec![("data", None, None, data.as_bytes())];
        for file in files {
            parts.push(("files", Some(file), Some("application/pdf"), b"%PDF-1.4"));
        }
        self.multipart(Method::POST, "/api/materials/submissions", token, &parts)
            .await
    }
}
