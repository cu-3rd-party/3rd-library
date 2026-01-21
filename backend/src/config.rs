use envconfig::Envconfig;

#[derive(envconfig::Envconfig)]
pub struct Config {
    #[envconfig(from = "DATABASE_URL")]
    pub database_url: String,
    #[envconfig(from = "HMAC_KEY")]
    pub hmac_key: String,
}
