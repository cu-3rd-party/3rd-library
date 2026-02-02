use envconfig::Envconfig;

pub struct Config {
    pub db: DbConfig,
    pub jwt: JwtConfig,
    pub redis: RedisConfig,
}

impl Config {
    pub fn init_from_env() -> Result<Self, envconfig::Error> {
        Ok(Self {
            db: DbConfig::init_from_env()?,
            jwt: JwtConfig::init_from_env()?,
            redis: RedisConfig::init_from_env()?,
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

impl DbConfig {
    pub fn database_url(&self) -> String {
        return format!(
            "postgres://{}:{}@{}:{}/{}",
            self.db_user, self.db_password, self.db_host, self.db_port, self.db_name
        );
    }
}
