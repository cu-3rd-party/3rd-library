use axum::{Router, routing::{get, post}};
use tokio::net::TcpListener;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod auth;
mod config;
mod content;
mod engagement;
mod errors;
mod handlers;
mod models;
mod statistics;

use crate::auth::AuthService;
use crate::config::AppConfig;
use crate::content::ContentService;
use crate::engagement::EngagementService;
use crate::handlers::{AppState, auth as auth_handlers, content as content_handlers, engagement as engagement_handlers};
use crate::statistics::StatisticsService;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = AppConfig::from_env();
    info!(?config, "backend starting");

    let content_service = ContentService::new(config.content_storage_dir.clone())
        .await
        .expect("failed to initialize content storage");

    let state = std::sync::Arc::new(AppState {
        auth: AuthService::new(config.jwt_secret.clone(), config.jwt_ttl),
        content: content_service,
        engagement: EngagementService::new(),
        statistics: StatisticsService::new(),
        max_upload_bytes: config.max_upload_bytes,
    });

    let app = Router::new()
        .route("/health", get(health))
        .route("/auth/register", post(auth_handlers::register))
        .route("/auth/login", post(auth_handlers::login))
        .route(
            "/contents",
            get(content_handlers::list_contents).post(content_handlers::upload_content),
        )
        .route("/contents/:id", get(content_handlers::get_content))
        .route("/contents/:id/download", get(content_handlers::download_content))
        .route("/contents/:id/stats", get(content_handlers::get_stats))
        .route(
            "/contents/:id/comments",
            get(engagement_handlers::list_comments).post(engagement_handlers::add_comment),
        )
        .route("/contents/:id/likes", post(engagement_handlers::set_like))
        .route("/contents/:id/engagement", get(engagement_handlers::get_engagement))
        .with_state(state)
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    let listener = TcpListener::bind(&config.listen_addr)
        .await
        .expect("failed to bind address");
    info!("serving at {}", config.listen_addr);
    axum::serve(listener, app).await.expect("server failed");
}

async fn health() -> &'static str {
    "ok"
}
