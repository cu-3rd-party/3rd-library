use std::env;
use std::net::SocketAddr;

use anyhow::Context;
use library_infrastructure::{InfrastructureConfig, RateLimitConfig, S3Config, SmtpConfig};

/// Настройки из переменных окружения (+ `.env` при локальном запуске). Имена — как у прежнего бэкенда.
#[derive(Clone)]
pub struct Config {
    /// `LISTEN_ADDR`, по умолчанию `0.0.0.0:8080`.
    pub listen_addr: SocketAddr,
    /// `METRICS_ADDR` — отдельный порт для `/metrics`, по умолчанию `0.0.0.0:9091`.
    pub metrics_addr: SocketAddr,
    /// `CORS_ALLOWED_ORIGIN`: пусто или `*` — любой.
    pub cors_allowed_origin: String,
    pub infrastructure: InfrastructureConfig,
}

impl Config {
    pub const DEFAULT_LISTEN_ADDR: &str = "0.0.0.0:8080";
    pub const DEFAULT_METRICS_ADDR: &str = "0.0.0.0:9091";

    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            listen_addr: parse("LISTEN_ADDR", Self::DEFAULT_LISTEN_ADDR)?,
            metrics_addr: parse("METRICS_ADDR", Self::DEFAULT_METRICS_ADDR)?,
            cors_allowed_origin: var("CORS_ALLOWED_ORIGIN", "*"),
            infrastructure: InfrastructureConfig {
                database_url: database_url(),
                run_migrations: true,
                hmac_key: var("HMAC_KEY", "dev"),
                rate_limit: RateLimitConfig {
                    capacity: parse("RATE_LIMIT_CAPACITY", "60")?,
                    refill_per_sec: parse("RATE_LIMIT_REFILL_PER_SEC", "30")?,
                },
                s3: S3Config {
                    access_key: var("S3_ACCESS_KEY", ""),
                    secret_key: var("S3_SECRET_KEY", ""),
                    // self-service отдаёт адрес как S3_URL
                    endpoint: env::var("S3_ENDPOINT").unwrap_or_else(|_| var("S3_URL", "")),
                    region: var("S3_REGION", "us-east-1"),
                    bucket_name: var("S3_BUCKET_NAME", "3rd-library"),
                },
                smtp: smtp()?,
            },
        })
    }
}

/// `DATABASE_URL` или, как раньше, из `POSTGRES_HOST`/`PORT`/`DB`/`USER`/`PASSWORD`.
fn database_url() -> String {
    env::var("DATABASE_URL").unwrap_or_else(|_| {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            var("POSTGRES_USER", "admin"),
            var("POSTGRES_PASSWORD", "admin"),
            var("POSTGRES_HOST", "localhost"),
            var("POSTGRES_PORT", "5432"),
            var("POSTGRES_DB", "db"),
        )
    })
}

/// SMTP, если задан `SMTP_HOST` и сборка с фичей `smtp`; иначе коды подтверждения только пишутся в лог.
fn smtp() -> anyhow::Result<Option<SmtpConfig>> {
    let host = env::var("SMTP_HOST").unwrap_or_default();
    if !cfg!(feature = "smtp") || host.is_empty() {
        return Ok(None);
    }
    Ok(Some(SmtpConfig {
        host,
        port: parse("SMTP_PORT", "465")?,
        user: var("SMTP_USER", "admin"),
        password: var("SMTP_PASSWORD", "admin"),
    }))
}

fn var(name: &str, default: &str) -> String {
    env::var(name).unwrap_or_else(|_| default.to_owned())
}

fn parse<T>(name: &str, default: &str) -> anyhow::Result<T>
where
    T: std::str::FromStr,
    T::Err: std::error::Error + Send + Sync + 'static,
{
    let value = var(name, default);
    value.parse().with_context(|| format!("invalid {name}: {value}"))
}
