use crate::config::Config;
use anyhow::Context;
use axum::Router;
use redis::aio::ConnectionManager;
use sqlx::PgPool;
use std::sync::Arc;

// Utility modules.

mod error;

mod extractor;

mod types;

// Api

mod articles;
mod profiles;
mod users;

pub use error::{Error, ResultExt};

pub type Result<T, E = Error> = std::result::Result<T, E>;

use crate::metrics;
use axum::http::HeaderValue;
use axum::routing::get;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

#[derive(Clone)]
struct ApiContext {
    config: Arc<Config>,
    db: PgPool,
    redis: ConnectionManager,
}

pub async fn serve(config: Config, db: PgPool, redis: ConnectionManager) -> anyhow::Result<()> {
    metrics::start_business_metrics_updater(db.clone());

    let cors = match config.cors.allowed_origin.as_deref() {
        Some(origin) if !origin.is_empty() => CorsLayer::new()
            .allow_origin(
                HeaderValue::from_str(origin).expect("CORS_ALLOWED_ORIGIN must be a valid origin"),
            )
            .allow_methods(tower_http::cors::Any)
            .allow_headers(tower_http::cors::Any),
        _ => CorsLayer::new()
            .allow_origin(tower_http::cors::Any)
            .allow_methods(tower_http::cors::Any)
            .allow_headers(tower_http::cors::Any),
    };

    let app = api_router()
        .with_state(ApiContext {
            config: Arc::new(config),
            db,
            redis: redis,
        })
        .layer(cors)
        .layer(axum::middleware::from_fn(metrics::metrics_middleware))
        .layer(TraceLayer::new_for_http());

    let metrics_app = Router::new().route("/metrics", get(metrics::metrics_handler));
    tokio::spawn(async move {
        let listener = tokio::net::TcpListener::bind("0.0.0.0:9091")
            .await
            .context("failed to bind metrics server")
            .expect("metrics bind");

        if let Err(err) = axum::serve(listener, metrics_app).await {
            log::error!("metrics server stopped: {err}");
        }
    });

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .context("failed to bind HTTP server")?;

    axum::serve(listener, app)
        .await
        .context("error running HTTP server")
}

fn api_router() -> Router<ApiContext> {
    users::router()
        .merge(profiles::router())
        .merge(articles::router())
}
