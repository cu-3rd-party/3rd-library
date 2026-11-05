/// Настройки инфраструктуры (собираются в web из переменных окружения).
#[derive(Debug, Clone)]
pub struct InfrastructureConfig {
    pub database_url: String,
    /// Применять миграции при старте.
    pub run_migrations: bool,
    /// Ключ подписи access-токенов (HMAC-SHA-384).
    pub hmac_key: String,
    pub rate_limit: RateLimitConfig,
    pub s3: S3Config,
    /// `None` — письма не отправляются (сборка без фичи `smtp`).
    pub smtp: Option<SmtpConfig>,
}

/// Token bucket на IP (в памяти реплики): ёмкость и пополнение в секунду; ноль в любом — ограничение выключено.
#[derive(Debug, Clone, Copy)]
pub struct RateLimitConfig {
    pub capacity: u64,
    pub refill_per_sec: u64,
}

#[derive(Debug, Clone)]
pub struct S3Config {
    pub access_key: String,
    pub secret_key: String,
    pub endpoint: String,
    pub region: String,
    pub bucket_name: String,
}

#[derive(Debug, Clone)]
pub struct SmtpConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
}
