use envconfig::Envconfig;

pub struct Config {
    pub db: DbConfig,
    pub jwt: JwtConfig,
    pub redis: RedisConfig,
    pub rate_limit: RateLimitConfig,
    pub cors: CorsConfig,
    pub storage: StorageConfig,
}

impl Config {
    pub fn init_from_env() -> Result<Self, envconfig::Error> {
        Ok(Self {
            db: DbConfig::init_from_env()?,
            jwt: JwtConfig::init_from_env()?,
            redis: RedisConfig::init_from_env()?,
            rate_limit: RateLimitConfig::init_from_env()?,
            cors: CorsConfig::init_from_env()?,
            storage: StorageConfig::init_from_env()?,
        })
    }
}

#[derive(envconfig::Envconfig)]
pub struct JwtConfig {
    #[envconfig(from = "HMAC_KEY")]
    pub hmac_key: String,
}

#[derive(envconfig::Envconfig)]
pub struct DbConfig {
    #[envconfig(from = "POSTGRES_HOST")]
    pub db_host: String,

    #[envconfig(from = "POSTGRES_PORT")]
    pub db_port: String,

    #[envconfig(from = "POSTGRES_DB")]
    pub db_name: String,

    #[envconfig(from = "POSTGRES_USER")]
    pub db_user: String,

    #[envconfig(from = "POSTGRES_PASSWORD")]
    pub db_password: String,
}

#[derive(envconfig::Envconfig)]
pub struct RedisConfig {
    #[envconfig(from = "REDIS_URL")]
    pub url: String,
}

#[derive(envconfig::Envconfig)]
pub struct RateLimitConfig {
    #[envconfig(from = "RATE_LIMIT_CAPACITY", default = "60")]
    pub capacity: u64,

    #[envconfig(from = "RATE_LIMIT_REFILL_PER_SEC", default = "30")]
    pub refill_per_sec: u64,
}

#[derive(envconfig::Envconfig)]
pub struct CorsConfig {
    #[envconfig(from = "CORS_ALLOWED_ORIGIN")]
    pub allowed_origin: Option<String>,
}

#[derive(envconfig::Envconfig)]
pub struct StorageConfig {
    #[envconfig(from = "PFP_UPLOAD_DIR")]
    pub pfp_upload_dir: String,
}

impl DbConfig {
    pub fn database_url(&self) -> String {
        return format!(
            "postgres://{}:{}@{}:{}/{}",
            self.db_user, self.db_password, self.db_host, self.db_port, self.db_name
        );
    }
}
