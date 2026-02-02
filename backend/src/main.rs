use anyhow::Context;
use redis::aio::ConnectionManager;
use redis::Client;
use sqlx::postgres::PgPoolOptions;

use backend::config::Config;
use backend::http;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let config = Config::init_from_env()?;

    let db = PgPoolOptions::new()
        .max_connections(50)
        .connect(&config.db.database_url())
        .await
        .context("could not connect to database_url")?;

    let redis_client = Client::open(config.redis.url.clone())
        .context("could not build Redis client")?;
    let redis = ConnectionManager::new(redis_client)
        .await
        .context("could not connect to Redis")?;

    if let Err(err) = sqlx::migrate!().run(&db).await {
        log::warn!("skipping migrations: {err}");
    }

    http::serve(config, db, redis).await?;

    Ok(())
}
