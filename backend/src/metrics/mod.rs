use once_cell::sync::Lazy;
use prometheus::{HistogramOpts, HistogramVec, IntCounter, IntCounterVec, IntGauge, Registry};

mod business;
mod handler;
mod middleware;
pub use business::start_business_metrics_updater;
pub use handler::metrics_handler;
pub use middleware::metrics_middleware;

static REGISTRY: Lazy<Registry> = Lazy::new(Registry::new);

static HTTP_REQUESTS_TOTAL: Lazy<IntCounterVec> = Lazy::new(|| {
    let metric = IntCounterVec::new(
        prometheus::opts!("http_requests_total", "Total HTTP requests"),
        &["method", "status", "route"],
    )
    .expect("http_requests_total");
    REGISTRY
        .register(Box::new(metric.clone()))
        .expect("register http_requests_total");
    metric
});

static HTTP_REQUEST_DURATION_SECONDS: Lazy<HistogramVec> = Lazy::new(|| {
    let metric = HistogramVec::new(
        HistogramOpts::new(
            "http_request_duration_seconds",
            "HTTP request latency in seconds",
        ),
        &["method", "status", "route"],
    )
    .expect("http_request_duration_seconds");
    REGISTRY
        .register(Box::new(metric.clone()))
        .expect("register http_request_duration_seconds");
    metric
});

static DB_QUERIES_TOTAL: Lazy<IntCounter> = Lazy::new(|| {
    let metric =
        IntCounter::new("db_queries_total", "Total DB queries executed").expect("db_queries_total");
    REGISTRY
        .register(Box::new(metric.clone()))
        .expect("register db_queries_total");
    metric
});

static USERS_TOTAL: Lazy<IntGauge> = Lazy::new(|| {
    let metric = IntGauge::new("users_total", "Total users").expect("users_total");
    REGISTRY
        .register(Box::new(metric.clone()))
        .expect("register users_total");
    metric
});

static SUBMISSIONS_TOTAL: Lazy<IntGauge> = Lazy::new(|| {
    let metric =
        IntGauge::new("submissions_total", "Total submissions").expect("submissions_total");
    REGISTRY
        .register(Box::new(metric.clone()))
        .expect("register submissions_total");
    metric
});

static MATERIAL_FILES_TOTAL: Lazy<IntGauge> = Lazy::new(|| {
    let metric = IntGauge::new("materials_files_total", "Total materials_files")
        .expect("materials_files_total");
    REGISTRY
        .register(Box::new(metric.clone()))
        .expect("register materials_files_total");
    metric
});

static MATERIALS_TOTAL: Lazy<IntGauge> = Lazy::new(|| {
    let metric = IntGauge::new("materials_total", "Total materials").expect("materials_total");
    REGISTRY
        .register(Box::new(metric.clone()))
        .expect("register materials_total");
    metric
});

static USERS_CREATED_TOTAL: Lazy<IntCounter> = Lazy::new(|| {
    let metric =
        IntCounter::new("users_created_total", "Users created").expect("users_created_total");
    REGISTRY
        .register(Box::new(metric.clone()))
        .expect("register users_created_total");
    metric
});

pub fn observe_db_query() {
    DB_QUERIES_TOTAL.inc();
}

pub fn record_user_created() {
    USERS_CREATED_TOTAL.inc();
}

pub fn record_material_file_created() {
    MATERIAL_FILES_TOTAL.inc();
}
