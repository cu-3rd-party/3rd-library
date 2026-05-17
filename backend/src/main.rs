use anyhow::Context;
use log::info;
use redis::Client;
use redis::aio::ConnectionManager;
use s3::creds::Credentials;
use s3::error::S3Error;
use s3::{Bucket, BucketConfiguration, Region};
use sqlx::Connection;
use sqlx::postgres::PgPoolOptions;

use backend::config::Config;
use backend::http;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    #[cfg(feature = "smtp")]
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .map_err(|_| anyhow::anyhow!("failed to install rustls crypto provider"))?;

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
        use redis::TypedCommands;
        let mut redis_client = redis_client.clone();
        redis_client.ping()?;
        info!("Redis client ping successful");
    }
    let redis = ConnectionManager::new(redis_client)
        .await
        .context("could not connect to Redis")?;
    info!("Successfully connected to redis");

    if let Err(err) = sqlx::migrate!().run(&db).await {
        log::warn!("skipping migrations: {err}");
    }

    let credentials = Credentials {
        access_key: Some(config.s3config.access_key.clone()),
        secret_key: Some(config.s3config.secret_key.clone()),
        security_token: None,
        session_token: None,
        expiration: None,
    };

    let region = Region::Custom {
        region: config.s3config.region.clone(),
        endpoint: config.s3config.endpoint.clone(),
    };

    let bucket = Bucket::new(
        &config.s3config.bucket_name,
        region.clone(),
        credentials.clone(),
    )?
    .with_path_style();
    match bucket.head_object("").await {
        Ok(_) => Ok::<(), anyhow::Error>(()),
        Err(S3Error::HttpFailWithBody(404, _)) => {
            Bucket::create_with_path_style(
                &config.s3config.bucket_name,
                region.clone(),
                credentials.clone(),
                BucketConfiguration::default(),
            )
            .await?;
            Ok(())
        }
        Err(e) => Err(e.into()),
    }?;

    http::serve(config, db, redis, bucket).await?;

    Ok(())
}
