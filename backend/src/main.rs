use anyhow::Context;
use log::info;
use redis::aio::ConnectionManager;
use redis::{Client, TypedCommands};
use sqlx::Connection;
use sqlx::postgres::PgPoolOptions;

use backend::config::Config;
use backend::http;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let config = Config::init_from_env();
    #[cfg(debug_assertions)]
    info!("Config: {:?}", &config);

    let db = PgPoolOptions::new()
        .max_connections(50)
        .connect(&config.db.database_url())
        .await
        .context("could not connect to database_url")?;
    #[cfg(debug_assertions)]
    {
        db.acquire().await?.ping().await?;
        info!("Database ping successful");
    }
    info!("Successfully connected to database");

    let redis_client =
        Client::open(config.redis.url.clone()).context("could not build Redis client")?;
    #[cfg(debug_assertions)]
    {
        let mut redis_client = redis_client.clone();
        redis_client.ping()?;
        info!("Redis ping successful");
    }
    info!("Successfully connected to database");
    let redis = ConnectionManager::new(redis_client)
        .await
        .context("could not connect to Redis")?;

    if let Err(err) = sqlx::migrate!().run(&db).await {
        log::warn!("skipping migrations: {err}");
    }

    http::serve(config, db, redis).await?;

    Ok(())
}
