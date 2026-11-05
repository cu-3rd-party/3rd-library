//! HTTP-метрики Prometheus (в общем реестре инфраструктуры) и `/metrics`.

use std::sync::LazyLock;
use std::time::Instant;

use axum::body::Body;
use axum::extract::MatchedPath;
use axum::http::{Request, StatusCode, header};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use library_infrastructure::metrics::registry;
use prometheus::{Encoder, HistogramOpts, HistogramVec, IntCounterVec, TextEncoder};

const LABELS: &[&str] = &["method", "status", "route"];

static HTTP_REQUESTS_TOTAL: LazyLock<IntCounterVec> = LazyLock::new(|| {
    let metric = IntCounterVec::new(prometheus::opts!("http_requests_total", "Total HTTP requests"), LABELS)
        .expect("http_requests_total");
    registry()
        .register(Box::new(metric.clone()))
        .expect("register http_requests_total");
    metric
});

static HTTP_REQUEST_DURATION_SECONDS: LazyLock<HistogramVec> = LazyLock::new(|| {
    let metric = HistogramVec::new(
        HistogramOpts::new("http_request_duration_seconds", "HTTP request latency in seconds"),
        LABELS,
    )
    .expect("http_request_duration_seconds");
    registry()
        .register(Box::new(metric.clone()))
        .expect("register http_request_duration_seconds");
    metric
});

pub async fn metrics_middleware(request: Request<Body>, next: Next) -> Response {
    let method = request.method().as_str().to_owned();
    let route = request
        .extensions()
        .get::<MatchedPath>()
        .map_or_else(|| "unknown".to_owned(), |path| path.as_str().to_owned());
    let start = Instant::now();

    let response = next.run(request).await;

    let status = response.status().as_u16().to_string();
    let labels = [method.as_str(), status.as_str(), route.as_str()];
    HTTP_REQUESTS_TOTAL.with_label_values(&labels).inc();
    HTTP_REQUEST_DURATION_SECONDS
        .with_label_values(&labels)
        .observe(start.elapsed().as_secs_f64());
    response
}

pub async fn metrics_handler() -> Response {
    let mut buffer = Vec::new();
    if TextEncoder::new().encode(&registry().gather(), &mut buffer).is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }
    ([(header::CONTENT_TYPE, "text/plain; version=0.0.4")], buffer).into_response()
}
