use crate::config::Config;
use anyhow::Context;
use axum::Router;
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

use tower_http::trace::TraceLayer;

#[derive(Clone)]
struct ApiContext {
    config: Arc<Config>,
    db: PgPool,
}

pub async fn serve(config: Config, db: PgPool) -> anyhow::Result<()> {
    let app = api_router()
        .with_state(ApiContext {
            config: Arc::new(config),
            db,
        })
        .layer(TraceLayer::new_for_http());

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
