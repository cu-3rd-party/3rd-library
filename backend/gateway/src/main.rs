use axum::{Router, routing::get, routing::post};
use tokio::net::TcpListener;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod clients;
mod config;
mod errors;
mod handlers;
mod models;
mod proto;

use crate::clients::Clients;
use crate::config::GatewayConfig;
use crate::handlers::{AppState, auth, content, engagement};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = GatewayConfig::from_env();
    tracing::info!(?config, "gateway starting");

    let clients = Clients::new(&config)
        .await
        .expect("failed to connect to gRPC services");
    let state = std::sync::Arc::new(AppState::new(clients, config.max_upload_bytes));

    let app = Router::new()
        .route("/health", get(health))
        .route("/auth/register", post(auth::register))
        .route("/auth/login", post(auth::login))
        .route(
            "/contents",
            get(content::list_contents).post(content::upload_content),
        )
        .route("/contents/:id", get(content::get_content))
        .route("/contents/:id/download", get(content::download_content))
        .route("/contents/:id/stats", get(content::get_stats))
        .route(
            "/contents/:id/comments",
            get(engagement::list_comments).post(engagement::add_comment),
        )
        .route("/contents/:id/likes", post(engagement::set_like))
        .route("/contents/:id/engagement", get(engagement::get_engagement))
        .with_state(state)
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    let addr = "0.0.0.0:8080";
    let listener = TcpListener::bind(addr)
        .await
        .expect("failed to bind address");
    axum::serve(listener, app).await.expect("server failed");
}

async fn health() -> &'static str {
    "ok"
}
