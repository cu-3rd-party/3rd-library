use sqlx::PgPool;
use tokio::time::sleep;

use crate::constants::{BUSINESS_REFRESH_INTERVAL, UPLOAD_DIR};

use super::{
    ARTICLES_TOTAL, ATTACHMENTS_TOTAL, ATTACHMENTS_TOTAL_BYTES, COMMENTS_TOTAL, USERS_TOTAL,
    observe_db_query,
};

pub fn start_business_metrics_updater(db: PgPool) {
    tokio::spawn(async move {
        loop {
            if let Err(err) = refresh_business_metrics(&db).await {
                log::warn!("failed to refresh business metrics: {err}");
            }
            sleep(BUSINESS_REFRESH_INTERVAL).await;
        }
    });
}

async fn refresh_business_metrics(db: &PgPool) -> Result<(), sqlx::Error> {
    observe_db_query();
    let users: Option<i64> = sqlx::query_scalar!(r#"select count(*) from "user""#)
        .fetch_one(db)
        .await?;
    USERS_TOTAL.set(users.unwrap_or(0) as i64);

    observe_db_query();
    let articles: Option<i64> = sqlx::query_scalar!("select count(*) from article")
        .fetch_one(db)
        .await?;
    ARTICLES_TOTAL.set(articles.unwrap_or(0) as i64);

    observe_db_query();
    let comments: Option<i64> = sqlx::query_scalar!("select count(*) from article_comment")
        .fetch_one(db)
        .await?;
    COMMENTS_TOTAL.set(comments.unwrap_or(0) as i64);

    observe_db_query();
    let attachments: Option<i64> = sqlx::query_scalar!("select count(*) from attachment")
        .fetch_one(db)
        .await?;
    ATTACHMENTS_TOTAL.set(attachments.unwrap_or(0) as i64);

    let bytes = attachment_bytes().await.unwrap_or(0);
    ATTACHMENTS_TOTAL_BYTES.set(bytes as i64);

    Ok(())
}

async fn attachment_bytes() -> Option<u64> {
    let mut total = 0u64;
    let mut entries = tokio::fs::read_dir(UPLOAD_DIR).await.ok()?;
    while let Ok(Some(entry)) = entries.next_entry().await {
        let metadata = match entry.metadata().await {
            Ok(metadata) => metadata,
            Err(_) => continue,
        };
        if metadata.is_file() {
            total = total.saturating_add(metadata.len());
        }
    }
    Some(total)
}
