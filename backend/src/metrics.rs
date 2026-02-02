use axum::body::Body;
use axum::extract::MatchedPath;
use axum::http::{HeaderValue, Request, StatusCode};
use axum::middleware::Next;
use axum::response::Response;
use once_cell::sync::Lazy;
use prometheus::{
    Encoder, HistogramOpts, HistogramVec, IntCounter, IntCounterVec, IntGauge, Registry,
    TextEncoder,
};
use sqlx::PgPool;
use std::time::Instant;
use tokio::time::sleep;

use crate::constants::{BUSINESS_REFRESH_INTERVAL, UPLOAD_DIR};

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

static ARTICLES_TOTAL: Lazy<IntGauge> = Lazy::new(|| {
    let metric = IntGauge::new("articles_total", "Total articles").expect("articles_total");
    REGISTRY
        .register(Box::new(metric.clone()))
        .expect("register articles_total");
    metric
});

static COMMENTS_TOTAL: Lazy<IntGauge> = Lazy::new(|| {
    let metric = IntGauge::new("comments_total", "Total comments").expect("comments_total");
    REGISTRY
        .register(Box::new(metric.clone()))
        .expect("register comments_total");
    metric
});

static ATTACHMENTS_TOTAL: Lazy<IntGauge> = Lazy::new(|| {
    let metric =
        IntGauge::new("attachments_total", "Total attachments").expect("attachments_total");
    REGISTRY
        .register(Box::new(metric.clone()))
        .expect("register attachments_total");
    metric
});

static ATTACHMENTS_TOTAL_BYTES: Lazy<IntGauge> = Lazy::new(|| {
    let metric = IntGauge::new("attachments_total_bytes", "Total attachment size in bytes")
        .expect("attachments_total_bytes");
    REGISTRY
        .register(Box::new(metric.clone()))
        .expect("register attachments_total_bytes");
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

static ARTICLES_CREATED_TOTAL: Lazy<IntCounter> = Lazy::new(|| {
    let metric = IntCounter::new("articles_created_total", "Articles created")
        .expect("articles_created_total");
    REGISTRY
        .register(Box::new(metric.clone()))
        .expect("register articles_created_total");
    metric
});

static COMMENTS_CREATED_TOTAL: Lazy<IntCounter> = Lazy::new(|| {
    let metric = IntCounter::new("comments_created_total", "Comments created")
        .expect("comments_created_total");
    REGISTRY
        .register(Box::new(metric.clone()))
        .expect("register comments_created_total");
    metric
});

static ATTACHMENTS_CREATED_TOTAL: Lazy<IntCounter> = Lazy::new(|| {
    let metric = IntCounter::new("attachments_created_total", "Attachments created")
        .expect("attachments_created_total");
    REGISTRY
        .register(Box::new(metric.clone()))
        .expect("register attachments_created_total");
    metric
});

pub fn observe_db_query() {
    DB_QUERIES_TOTAL.inc();
}

pub fn record_user_created() {
    USERS_CREATED_TOTAL.inc();
}

pub fn record_article_created() {
    ARTICLES_CREATED_TOTAL.inc();
}

pub fn record_comment_created() {
    COMMENTS_CREATED_TOTAL.inc();
}

pub fn record_attachment_created() {
    ATTACHMENTS_CREATED_TOTAL.inc();
}

pub async fn metrics_middleware(req: Request<Body>, next: Next) -> Response {
    let method = req.method().as_str().to_string();
    let route = req
        .extensions()
        .get::<MatchedPath>()
        .map(|matched| matched.as_str().to_string())
        .unwrap_or_else(|| "unknown".to_string());
    let start = Instant::now();
    let response = next.run(req).await;
    let status = response.status().as_u16().to_string();

    HTTP_REQUESTS_TOTAL
        .with_label_values(&[method.as_str(), status.as_str(), route.as_str()])
        .inc();
    HTTP_REQUEST_DURATION_SECONDS
        .with_label_values(&[method.as_str(), status.as_str(), route.as_str()])
        .observe(start.elapsed().as_secs_f64());

    response
}

pub async fn metrics_handler() -> Result<Response, StatusCode> {
    let encoder = TextEncoder::new();
    let metric_families = REGISTRY.gather();
    let mut buffer = Vec::new();
    encoder
        .encode(&metric_families, &mut buffer)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let body = String::from_utf8(buffer).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let mut response = Response::new(Body::from(body));
    response.headers_mut().insert(
        axum::http::header::CONTENT_TYPE,
        HeaderValue::from_static("text/plain; version=0.0.4"),
    );

    Ok(response)
}

pub fn start_business_metrics_updater(db: PgPool) {
    tokio::spawn(async move {
        loop {
            if let Err(err) = refresh_business_metrics(&db).await {
                log::warn!("failed to refresh business metrics: {err}");
            }
            sleep(BUSINESS_REFRESH_INTERVAL).await;
        }
    });
}

async fn refresh_business_metrics(db: &PgPool) -> Result<(), sqlx::Error> {
    observe_db_query();
    let users: Option<i64> = sqlx::query_scalar!(r#"select count(*) from "user""#)
        .fetch_one(db)
        .await?;
    USERS_TOTAL.set(users.unwrap_or(0) as i64);

    observe_db_query();
    let articles: Option<i64> = sqlx::query_scalar!("select count(*) from article")
        .fetch_one(db)
        .await?;
    ARTICLES_TOTAL.set(articles.unwrap_or(0) as i64);

    observe_db_query();
    let comments: Option<i64> = sqlx::query_scalar!("select count(*) from article_comment")
        .fetch_one(db)
        .await?;
    COMMENTS_TOTAL.set(comments.unwrap_or(0) as i64);

    observe_db_query();
    let attachments: Option<i64> = sqlx::query_scalar!("select count(*) from attachment")
        .fetch_one(db)
        .await?;
    ATTACHMENTS_TOTAL.set(attachments.unwrap_or(0) as i64);

    let bytes = attachment_bytes().await.unwrap_or(0);
    ATTACHMENTS_TOTAL_BYTES.set(bytes as i64);

    Ok(())
}

async fn attachment_bytes() -> Option<u64> {
    let mut total = 0u64;
    let mut entries = tokio::fs::read_dir(UPLOAD_DIR).await.ok()?;
    while let Ok(Some(entry)) = entries.next_entry().await {
        let metadata = match entry.metadata().await {
            Ok(metadata) => metadata,
            Err(_) => continue,
        };
        if metadata.is_file() {
            total = total.saturating_add(metadata.len());
        }
    }
    Some(total)
}
