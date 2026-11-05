use async_trait::async_trait;
use bytes::Bytes;
use library_application::ports::{ExternalError, ExternalResult, FileStorage};
use s3::creds::Credentials;
use s3::error::S3Error;
use s3::{Bucket, BucketConfiguration, Region};

use crate::S3Config;

/// Файлы в S3-совместимом хранилище (path-style: MinIO, RustFS).
pub struct S3FileStorage {
    bucket: Box<Bucket>,
}

impl S3FileStorage {
    /// Открывает бакет и создаёт его, если его ещё нет (локально и в docker compose).
    pub async fn connect(config: &S3Config) -> Result<Self, S3Error> {
        let credentials = Credentials {
            access_key: Some(config.access_key.clone()),
            secret_key: Some(config.secret_key.clone()),
            security_token: None,
            session_token: None,
            expiration: None,
        };
        let region = Region::Custom {
            region: config.region.clone(),
            endpoint: config.endpoint.clone(),
        };

        let bucket = Bucket::new(&config.bucket_name, region.clone(), credentials.clone())?.with_path_style();
        // листинг внутри бакета требует прав только на свой бакет (в self-service бакет уже создан платформой)
        match bucket.list_page(String::new(), None, None, None, Some(1)).await {
            Ok(_) => {}
            Err(S3Error::HttpFailWithBody(404, _)) => {
                tracing::info!(bucket = %config.bucket_name, "creating S3 bucket");
                Bucket::create_with_path_style(
                    &config.bucket_name,
                    region,
                    credentials,
                    BucketConfiguration::default(),
                )
                .await?;
            }
            Err(error) => tracing::warn!(%error, bucket = %config.bucket_name, "could not check the S3 bucket"),
        }
        Ok(Self { bucket })
    }
}

#[async_trait]
impl FileStorage for S3FileStorage {
    async fn put(&self, key: &str, content: Bytes, content_type: &str) -> ExternalResult<()> {
        self.bucket
            .put_object_with_content_type(key, &content, content_type)
            .await
            .map_err(ExternalError::new)?;
        Ok(())
    }
}
