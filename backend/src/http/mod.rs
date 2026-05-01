use crate::config::Config;
use anyhow::Context;
use axum::Router;
use redis::aio::ConnectionManager;
use sqlx::PgPool;
use std::net::SocketAddr;
use std::sync::Arc;

mod error;
mod extractor;
mod rate_limit;
mod types;

mod auth;
pub mod materials;
mod moderation;
mod users;

pub use error::{Error, ResultExt};

pub type Result<T, E = Error> = std::result::Result<T, E>;

use crate::metrics;
use axum::http::HeaderValue;
use axum::routing::get;
use log::info;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

#[derive(Clone)]
pub struct ApiContext {
    config: Arc<Config>,
    db: PgPool,
    redis: ConnectionManager,
    rate_limit_ttl_seconds: u64,
}

pub async fn serve(config: Config, db: PgPool, redis: ConnectionManager) -> anyhow::Result<()> {
    metrics::start_business_metrics_updater(db.clone());

    let cors = match config.cors.allowed_origin.as_str() {
        "" => CorsLayer::new()
            .allow_origin(tower_http::cors::Any)
            .allow_methods(tower_http::cors::Any)
            .allow_headers(tower_http::cors::Any),
        origin => CorsLayer::new()
            .allow_origin(
                HeaderValue::from_str(origin).expect("CORS_ALLOWED_ORIGIN must be a valid origin"),
            )
            .allow_methods(tower_http::cors::Any)
            .allow_headers(tower_http::cors::Any),
    };

    let rate_limit_ttl_seconds = rate_limit::bucket_ttl_seconds(
        config.rate_limit.capacity,
        config.rate_limit.refill_per_sec,
    );

    let context = ApiContext {
        config: Arc::new(config),
        db,
        redis,
        rate_limit_ttl_seconds,
    };

    let app = api_router()
        .with_state(context.clone())
        .layer(axum::middleware::from_fn_with_state(
            context.clone(),
            rate_limit::rate_limit_middleware,
        ))
        .layer(cors)
        .layer(axum::middleware::from_fn(metrics::metrics_middleware))
        .layer(TraceLayer::new_for_http());

    let metrics_app = Router::new().route("/metrics", get(metrics::metrics_handler));
    tokio::spawn(async move {
        let listener = tokio::net::TcpListener::bind("0.0.0.0:9091")
            .await
            .context("failed to bind metrics server")
            .expect("metrics bind");

        info!(
            "metrics server listening on {}",
            listener.local_addr().unwrap()
        );
        if let Err(err) = axum::serve(listener, metrics_app).await {
            log::error!("metrics server stopped: {err}");
        }
    });

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .context("failed to bind HTTP server")?;

    info!("HTTP server listening on {}", listener.local_addr()?);
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .context("error running HTTP server")
}

fn api_router() -> Router<ApiContext> {
    auth::router()
        .merge(users::router())
        .merge(materials::router())
        .merge(moderation::router())
}
