use std::net::SocketAddr;

use anyhow::Context;
use axum::Router;
use axum::routing::get;
use library_infrastructure::Infrastructure;
use library_infrastructure::metrics::start_business_metrics_updater;
use library_web::config::Config;
use library_web::{AppState, app, cors_layer, metrics};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let config = Config::from_env()?;

    // `library-web healthcheck` — для HEALTHCHECK в distroless-образе (там нет wget/curl)
    if std::env::args().nth(1).as_deref() == Some("healthcheck") {
        return healthcheck(&config).await;
    }

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info,sqlx=warn")))
        .init();
    // TLS для SMTP и S3; провайдер мог уже поставить кто-то из зависимостей
    rustls::crypto::aws_lc_rs::default_provider().install_default().ok();

    let infrastructure = Infrastructure::init(&config.infrastructure).await?;
    start_business_metrics_updater(infrastructure.db.clone());
    let app = app(AppState::new(infrastructure), cors_layer(&config.cors_allowed_origin)?);

    let metrics_listener = TcpListener::bind(config.metrics_addr)
        .await
        .context("failed to bind metrics server")?;
    tracing::info!("metrics on http://{}/metrics", metrics_listener.local_addr()?);
    tokio::spawn(async move {
        let metrics_app = Router::new().route("/metrics", get(metrics::metrics_handler));
        if let Err(error) = axum::serve(metrics_listener, metrics_app).await {
            tracing::error!(%error, "metrics server stopped");
        }
    });

    let listener = TcpListener::bind(config.listen_addr)
        .await
        .context("failed to bind HTTP server")?;
    tracing::info!(
        "listening on http://{}, docs on /api/docs/swagger-ui",
        listener.local_addr()?
    );
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>())
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c().await.ok();
    };
    #[cfg(unix)]
    let terminate = async {
        if let Ok(mut signal) = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()) {
            signal.recv().await;
        }
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        () = ctrl_c => {},
        () = terminate => {},
    }
}

async fn healthcheck(config: &Config) -> anyhow::Result<()> {
    let mut stream = TcpStream::connect(("127.0.0.1", config.listen_addr.port())).await?;
    stream
        .write_all(b"GET /api/health HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n")
        .await?;
    let mut response = String::new();
    stream.read_to_string(&mut response).await?;
    anyhow::ensure!(
        response.starts_with("HTTP/1.1 200"),
        "unhealthy: {}",
        response.lines().next().unwrap_or("")
    );
    Ok(())
}
