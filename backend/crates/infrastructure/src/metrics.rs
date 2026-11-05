//! Prometheus: общий реестр (HTTP-метрики добавляет web), счётчики запросов к БД и бизнес-метрики.

use std::sync::LazyLock;
use std::time::Duration;

use prometheus::{IntCounter, IntGauge, Registry};
use sqlx::PgPool;

const BUSINESS_REFRESH_INTERVAL: Duration = Duration::from_secs(30);

static REGISTRY: LazyLock<Registry> = LazyLock::new(Registry::new);

fn register<M: prometheus::core::Collector + Clone + 'static>(metric: M) -> M {
    REGISTRY
        .register(Box::new(metric.clone()))
        .expect("metric names are unique");
    metric
}

static DB_QUERIES_TOTAL: LazyLock<IntCounter> = LazyLock::new(|| {
    register(IntCounter::new("db_queries_total", "Total DB queries executed").expect("db_queries_total"))
});
static USERS_CREATED_TOTAL: LazyLock<IntCounter> =
    LazyLock::new(|| register(IntCounter::new("users_created_total", "Users created").expect("users_created_total")));
static USERS_TOTAL: LazyLock<IntGauge> =
    LazyLock::new(|| register(IntGauge::new("users_total", "Total users").expect("users_total")));
static SUBMISSIONS_TOTAL: LazyLock<IntGauge> =
    LazyLock::new(|| register(IntGauge::new("submissions_total", "Total submissions").expect("submissions_total")));
static MATERIALS_TOTAL: LazyLock<IntGauge> =
    LazyLock::new(|| register(IntGauge::new("materials_total", "Total materials").expect("materials_total")));
static MATERIAL_FILES_TOTAL: LazyLock<IntGauge> = LazyLock::new(|| {
    register(IntGauge::new("materials_files_total", "Total materials_files").expect("materials_files_total"))
});

/// Реестр, который отдаёт `/metrics`.
pub fn registry() -> &'static Registry {
    &REGISTRY
}

pub(crate) fn observe_db_query() {
    DB_QUERIES_TOTAL.inc();
}

pub(crate) fn record_user_created() {
    USERS_CREATED_TOTAL.inc();
}

/// Фоновое обновление числа пользователей, заявок, материалов и файлов.
pub fn start_business_metrics_updater(db: PgPool) {
    tokio::spawn(async move {
        loop {
            if let Err(error) = refresh_business_metrics(&db).await {
                tracing::warn!(%error, "failed to refresh business metrics");
            }
            tokio::time::sleep(BUSINESS_REFRESH_INTERVAL).await;
        }
    });
}

async fn refresh_business_metrics(db: &PgPool) -> Result<(), sqlx::Error> {
    for (gauge, table) in [
        (&USERS_TOTAL, "web_user"),
        (&SUBMISSIONS_TOTAL, "submission"),
        (&MATERIALS_TOTAL, "material"),
        (&MATERIAL_FILES_TOTAL, "material_file"),
    ] {
        observe_db_query();
        let count: i64 = sqlx::query_scalar(&format!("select count(*) from {table}"))
            .fetch_one(db)
            .await?;
        gauge.set(count);
    }
    Ok(())
}
