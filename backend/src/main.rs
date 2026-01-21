use anyhow::Context;
use envconfig::Envconfig;
use sqlx::postgres::PgPoolOptions;

use backend::config::Config;
use backend::http;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let config = Config::init_from_env()?;

    let db = PgPoolOptions::new()
        .max_connections(50)
        .connect(&config.database_url)
        .await
        .context("could not connect to database_url")?;

    if let Err(err) = sqlx::migrate!().run(&db).await {
        log::warn!("skipping migrations: {err}");
    }

    http::serve(config, db).await?;

    Ok(())
}
