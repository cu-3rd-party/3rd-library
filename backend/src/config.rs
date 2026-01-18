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

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::sync::{Mutex, OnceLock};

    fn env_lock() -> std::sync::MutexGuard<'static, ()> {
        static ENV_MUTEX: OnceLock<Mutex<()>> = OnceLock::new();
        ENV_MUTEX
            .get_or_init(|| Mutex::new(()))
            .lock()
            .expect("env mutex")
    }

    fn with_env(vars: &[(&str, Option<&str>)], f: impl FnOnce()) {
        let _guard = env_lock();
        let mut prev = Vec::with_capacity(vars.len());
        for (key, value) in vars {
            prev.push((*key, env::var(*key).ok()));
            match value {
                Some(val) => unsafe { env::set_var(*key, val) },
                None => unsafe { env::remove_var(*key) },
            }
        }

        f();

        for (key, value) in prev {
            match value {
                Some(val) => unsafe { env::set_var(key, val) },
                None => unsafe { env::remove_var(key) },
            }
        }
    }

    #[test]
    fn from_env_uses_defaults_on_invalid_jwt_ttl() {
        with_env(
            &[("JWT_TTL", Some("not-a-duration"))],
            || {
                let config = AppConfig::from_env();
                assert_eq!(config.jwt_ttl, Duration::from_secs(24 * 60 * 60));
            },
        );
    }

    #[test]
    fn from_env_respects_overrides() {
        with_env(
            &[
                ("BACKEND_ADDR", Some("127.0.0.1:9999")),
                ("MAX_UPLOAD_BYTES", Some("123")),
                ("JWT_SECRET", Some("test-secret")),
                ("JWT_TTL", Some("2h")),
                ("CONTENT_STORAGE_DIR", Some("/tmp/3rd-library-test")),
                ("DATABASE_URL", Some("postgres://user:pass@localhost:5432/testdb")),
            ],
            || {
                let config = AppConfig::from_env();
                assert_eq!(config.listen_addr, "127.0.0.1:9999");
                assert_eq!(config.max_upload_bytes, 123);
                assert_eq!(config.jwt_secret, "test-secret");
                assert_eq!(config.jwt_ttl, Duration::from_secs(2 * 60 * 60));
                assert_eq!(config.content_storage_dir, "/tmp/3rd-library-test");
                assert_eq!(config.database_url, "postgres://user:pass@localhost:5432/testdb");
            },
        );
    }
}
