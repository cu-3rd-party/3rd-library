use std::env;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub listen_addr: String,
    pub max_upload_bytes: usize,
    pub jwt_secret: String,
    pub jwt_ttl: Duration,
    pub content_storage_dir: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let jwt_ttl = env::var("JWT_TTL")
            .ok()
            .and_then(|value| humantime::parse_duration(&value).ok())
            .unwrap_or_else(|| Duration::from_secs(24 * 60 * 60));

        Self {
            listen_addr: env_or_default("BACKEND_ADDR", "0.0.0.0:8080"),
            max_upload_bytes: env_or_default("MAX_UPLOAD_BYTES", "10485760")
                .parse()
                .unwrap_or(10 * 1024 * 1024),
            jwt_secret: env_or_default("JWT_SECRET", "dev-secret"),
            jwt_ttl,
            content_storage_dir: env_or_default("CONTENT_STORAGE_DIR", "/tmp/3rd-library/content"),
        }
    }
}

fn env_or_default(key: &str, default_value: &str) -> String {
    env::var(key).unwrap_or_else(|_| default_value.to_string())
}
