use axum::{routing::get, Router};
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = GatewayConfig::from_env();
    tracing::info!(?config, "gateway starting");

    // Placeholder: initialize gRPC clients with tonic once proto stubs are generated.
    let app = Router::new().route("/health", get(health));

    let addr = "0.0.0.0:8080";
    let listener = TcpListener::bind(addr)
        .await
        .expect("failed to bind address");
    axum::serve(listener, app)
        .await
        .expect("server failed");
}

async fn health() -> &'static str {
    "ok"
}

#[derive(Debug)]
struct GatewayConfig {
    redis_url: String,
    kafka_brokers: String,
}

impl GatewayConfig {
    fn from_env() -> Self {
        Self {
            redis_url: env_or_default("REDIS_URL", "redis://localhost:6379/0"),
            kafka_brokers: env_or_default("KAFKA_BROKERS", "localhost:9092"),
        }
    }
}

fn env_or_default(key: &str, default_value: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| default_value.to_string())
}
