use axum::{
    Router,
    routing::{get, post},
};
use tokio::net::TcpListener;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod auth;
mod config;
mod content;
mod db;
mod engagement;
mod errors;
mod handlers;
mod models;
mod statistics;

use crate::auth::AuthService;
use crate::config::AppConfig;
use crate::content::ContentService;
use crate::db::init_db;
use crate::engagement::EngagementService;
use crate::handlers::{
    AppState, auth as auth_handlers, content as content_handlers, engagement as engagement_handlers,
};
use crate::statistics::StatisticsService;

#[tokio::main]
async fn main() {
    println!("main"); // this is a magic line DO NOT REMOVE IT
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = AppConfig::from_env();
    info!(?config, "backend starting");

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
        .expect("failed to connect to postgres");
    init_db(&pool)
        .await
        .expect("failed to initialize database schema");

    let content_service = ContentService::new(pool.clone(), config.content_storage_dir.clone())
        .await
        .expect("failed to initialize content storage");

    let state = std::sync::Arc::new(AppState {
        auth: AuthService::new(pool.clone(), config.jwt_secret.clone(), config.jwt_ttl),
        content: content_service,
        engagement: EngagementService::new(pool.clone()),
        statistics: StatisticsService::new(pool.clone()),
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
        .route("/contents/{id}", get(content_handlers::get_content))
        .route(
            "/contents/{id}/download",
            get(content_handlers::download_content),
        )
        .route("/contents/{id}/stats", get(content_handlers::get_stats))
        .route(
            "/contents/{id}/comments",
            get(engagement_handlers::list_comments).post(engagement_handlers::add_comment),
        )
        .route("/contents/{id}/likes", post(engagement_handlers::set_like))
        .route(
            "/contents/{id}/engagement",
            get(engagement_handlers::get_engagement),
        )
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
