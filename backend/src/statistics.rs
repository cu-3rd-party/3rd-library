use std::time::{SystemTime, UNIX_EPOCH};

use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub enum InteractionType {
    View,
    Download,
}

#[derive(Debug, Clone)]
pub struct ContentStats {
    pub views: i64,
    pub downloads: i64,
    pub unique_users: i64,
    pub last_interaction_at: i64,
}

#[derive(Debug)]
pub enum StatisticsError {
    InvalidInput(String),
    Db(String),
}

#[derive(Clone)]
pub struct StatisticsService {
    pool: PgPool,
}

impl StatisticsService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn record(
        &self,
        content_id: String,
        user_id: String,
        interaction_type: InteractionType,
        occurred_at: i64,
    ) -> Result<String, StatisticsError> {
        let content_id = content_id.trim().to_string();
        let user_id = user_id.trim().to_string();
        if content_id.is_empty() || user_id.is_empty() {
            return Err(StatisticsError::InvalidInput(
                "content_id and user_id required".to_string(),
            ));
        }
        let occurred_at = if occurred_at <= 0 {
            now_timestamp()
        } else {
            occurred_at
        };

        let interaction = match interaction_type {
            InteractionType::View => "view",
            InteractionType::Download => "download",
        };

        let recorded_id = Uuid::new_v4().simple().to_string();
        sqlx::query(
            r#"
            INSERT INTO interactions (id, content_id, user_id, interaction_type, occurred_at)
            VALUES ($1, $2, $3, $4, $5)
            "#,
        )
        .bind(&recorded_id)
        .bind(&content_id)
        .bind(&user_id)
        .bind(interaction)
        .bind(occurred_at)
        .execute(&self.pool)
        .await
        .map_err(|err| StatisticsError::Db(err.to_string()))?;

        Ok(recorded_id)
    }

    pub async fn stats_for(&self, content_id: String) -> Result<ContentStats, StatisticsError> {
        let content_id = content_id.trim().to_string();
        if content_id.is_empty() {
            return Err(StatisticsError::InvalidInput("content_id required".to_string()));
        }

        let row = sqlx::query_as::<_, StatsRow>(
            r#"
            SELECT
                COUNT(*) FILTER (WHERE interaction_type = 'view') AS views,
                COUNT(*) FILTER (WHERE interaction_type = 'download') AS downloads,
                COUNT(DISTINCT user_id) AS unique_users,
                COALESCE(MAX(occurred_at), 0) AS last_interaction_at
            FROM interactions
            WHERE content_id = $1
            "#,
        )
        .bind(&content_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|err| StatisticsError::Db(err.to_string()))?;

        Ok(ContentStats {
            views: row.views,
            downloads: row.downloads,
            unique_users: row.unique_users,
            last_interaction_at: row.last_interaction_at,
        })
    }
}

#[derive(Debug, FromRow)]
struct StatsRow {
    views: i64,
    downloads: i64,
    unique_users: i64,
    last_interaction_at: i64,
}

fn now_timestamp() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}
