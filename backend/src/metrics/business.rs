use sqlx::PgPool;
use tokio::time::sleep;

use crate::constants::BUSINESS_REFRESH_INTERVAL;

use super::{
    MATERIAL_FILES_TOTAL, MATERIALS_TOTAL, SUBMISSIONS_TOTAL, USERS_TOTAL, observe_db_query,
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
    let users: Option<i64> = sqlx::query_scalar(r#"select count(*) from "web_user""#)
        .fetch_one(db)
        .await?;
    USERS_TOTAL.set(users.unwrap_or(0) as i64);

    observe_db_query();
    let submissions: Option<i64> = sqlx::query_scalar("select count(*) from submission")
        .fetch_one(db)
        .await?;
    SUBMISSIONS_TOTAL.set(submissions.unwrap_or(0) as i64);

    observe_db_query();
    let materials: Option<i64> = sqlx::query_scalar("select count(*) from material")
        .fetch_one(db)
        .await?;
    MATERIALS_TOTAL.set(materials.unwrap_or(0) as i64);

    observe_db_query();
    let material_files: Option<i64> = sqlx::query_scalar("select count(*) from material_file")
        .fetch_one(db)
        .await?;
    MATERIAL_FILES_TOTAL.set(material_files.unwrap_or(0) as i64);

    Ok(())
}
