use std::time::Duration;

use envconfig::Envconfig;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub listen_addr: String,
    pub max_upload_bytes: usize,
    pub jwt_secret: String,
    pub jwt_ttl: Duration,
    pub content_storage_dir: String,
    pub database_url: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let raw = AppConfigEnv::init_from_env().expect("failed to load backend config");
        let jwt_ttl = humantime::parse_duration(&raw.jwt_ttl)
            .unwrap_or_else(|_| Duration::from_secs(24 * 60 * 60));

        Self {
            listen_addr: raw.listen_addr,
            max_upload_bytes: raw.max_upload_bytes,
            jwt_secret: raw.jwt_secret,
            jwt_ttl,
            content_storage_dir: raw.content_storage_dir,
            database_url: raw.database_url,
        }
    }
}

#[derive(Envconfig)]
struct AppConfigEnv {
    #[envconfig(from = "BACKEND_ADDR", default = "0.0.0.0:8080")]
    listen_addr: String,
    #[envconfig(from = "MAX_UPLOAD_BYTES", default = "10485760")]
    max_upload_bytes: usize,
    #[envconfig(from = "JWT_SECRET", default = "dev-secret")]
    jwt_secret: String,
    #[envconfig(from = "JWT_TTL", default = "24h")]
    jwt_ttl: String,
    #[envconfig(from = "CONTENT_STORAGE_DIR", default = "/tmp/3rd-library/content")]
    content_storage_dir: String,
    #[envconfig(
        from = "DATABASE_URL",
        default = "postgres://backend:backend@localhost:5432/backend"
    )]
    database_url: String,
}
