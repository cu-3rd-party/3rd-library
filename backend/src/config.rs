use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, ignore_errors = true)]
pub struct Config {
    #[command(flatten)]
    pub db: DbConfig,

    #[command(flatten)]
    pub jwt: JwtConfig,

    #[command(flatten)]
    pub redis: RedisConfig,

    #[command(flatten)]
    pub rate_limit: RateLimitConfig,

    #[command(flatten)]
    pub cors: CorsConfig,

    #[command(flatten)]
    pub storage: StorageConfig,

    #[cfg(feature = "smtp")]
    #[command(flatten)]
    pub smtp: SmtpConfig,

    #[command(flatten)]
    pub s3config: S3Config,
}

impl Config {
    pub fn init_from_env() -> Self {
        Self::parse()
    }
}

#[derive(Parser, Debug, Clone)]
#[group(id = "db")]
pub struct DbConfig {
    #[arg(env = "POSTGRES_HOST", default_value = "localhost")]
    pub db_host: String,

    #[arg(env = "POSTGRES_PORT", default_value = "5432")]
    pub db_port: u16,

    #[arg(env = "POSTGRES_DB", default_value = "db")]
    pub db_name: String,

    #[arg(env = "POSTGRES_USER", default_value = "admin")]
    pub db_user: String,

    #[arg(env = "POSTGRES_PASSWORD", default_value = "admin")]
    pub db_password: String,
}

impl DbConfig {
    pub fn database_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.db_user, self.db_password, self.db_host, self.db_port, self.db_name
        )
    }
}

#[derive(Parser, Debug, Clone)]
#[group(id = "jwt")]
pub struct JwtConfig {
    #[arg(env = "HMAC_KEY", default_value = "dev")]
    pub hmac_key: String,
}

#[derive(Parser, Debug, Clone)]
#[group(id = "redis")]
pub struct RedisConfig {
    #[arg(env = "REDIS_URL", default_value = "redis://localhost:6379")]
    pub url: String,
}

#[derive(Parser, Debug, Clone)]
#[group(id = "rate_limit")]
pub struct RateLimitConfig {
    #[arg(env = "RATE_LIMIT_CAPACITY", default_value = "60")]
    pub capacity: u64,

    #[arg(env = "RATE_LIMIT_REFILL_PER_SEC", default_value = "30")]
    pub refill_per_sec: u64,
}

#[derive(Parser, Debug, Clone)]
#[group(id = "cors")]
pub struct CorsConfig {
    #[arg(env = "CORS_ALLOWED_ORIGIN", default_value = "*")]
    pub allowed_origin: String,
}

#[derive(Parser, Debug, Clone)]
#[group(id = "storage")]
pub struct StorageConfig {
    #[arg(env = "PFP_UPLOAD_DIR", default_value = "/uploads")]
    pub pfp_upload_dir: String,
}

#[cfg(feature = "smtp")]
#[derive(Parser, Debug, Clone)]
#[group(id = "smtp")]
pub struct SmtpConfig {
    #[arg(env = "SMTP_HOST")]
    pub host: String,
    #[arg(env = "SMTP_PORT", default_value = "5432")]
    pub port: u16,
    #[arg(env = "SMTP_USER", default_value = "admin")]
    pub user: String,
    #[arg(env = "SMTP_PASSWORD", default_value = "admin")]
    pub password: String,
}

#[derive(Parser, Debug, Clone)]
#[group(id = "s3")]
pub struct S3Config {
    #[arg(env = "S3_ACCESS_KEY")]
    pub access_key: String,
    #[arg(env = "S3_SECRET_KEY")]
    pub secret_key: String,
    #[arg(env = "S3_ENDPOINT")]
    pub endpoint: String,
    #[arg(env = "S3_REGION", default_value = "us-east-1")]
    pub region: String,
    #[arg(env = "S3_BUCKET_NAME", default_value = "3rd-library")]
    pub bucket_name: String,
}
