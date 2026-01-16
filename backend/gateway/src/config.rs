use std::env;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct GatewayConfig {
    pub redis_url: String,
    pub kafka_brokers: String,
    pub auth_grpc_addr: String,
    pub content_grpc_addr: String,
    pub engagement_grpc_addr: String,
    pub statistics_grpc_addr: String,
    pub max_upload_bytes: usize,
}

impl GatewayConfig {
    pub fn from_env() -> Self {
        Self {
            redis_url: env_or_default("REDIS_URL", "redis://localhost:6379/0"),
            kafka_brokers: env_or_default("KAFKA_BROKERS", "localhost:9092"),
            auth_grpc_addr: env_or_default("AUTH_GRPC_ADDR", "http://auth:50051"),
            content_grpc_addr: env_or_default("CONTENT_GRPC_ADDR", "http://content:50055"),
            engagement_grpc_addr: env_or_default("ENGAGEMENT_GRPC_ADDR", "http://engagement:50054"),
            statistics_grpc_addr: env_or_default("STATISTICS_GRPC_ADDR", "http://statistics:50052"),
            max_upload_bytes: env_or_default("MAX_UPLOAD_BYTES", "10485760")
                .parse()
                .unwrap_or(10 * 1024 * 1024),
        }
    }
}

fn env_or_default(key: &str, default_value: &str) -> String {
    env::var(key).unwrap_or_else(|_| default_value.to_string())
}
