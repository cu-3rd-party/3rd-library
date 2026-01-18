use std::time::{SystemTime, UNIX_EPOCH};

use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Comment {
    pub id: String,
    pub content_id: String,
    pub user_id: String,
    pub body: String,
    pub created_at: i64,
}

#[derive(Debug, Clone)]
pub struct EngagementSummary {
    pub likes: i64,
    pub comments: i64,
    pub liked_by_user: bool,
}

#[derive(Debug)]
pub enum EngagementError {
    InvalidInput(String),
    Db(String),
}

#[derive(Clone)]
pub struct EngagementService {
    pool: PgPool,
}

impl EngagementService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn add_comment(
        &self,
        content_id: String,
        user_id: String,
        body: String,
        created_at: i64,
    ) -> Result<Comment, EngagementError> {
        let content_id = content_id.trim().to_string();
        let user_id = user_id.trim().to_string();
        let body = body.trim().to_string();
        if content_id.is_empty() || user_id.is_empty() || body.is_empty() {
            return Err(EngagementError::InvalidInput(
                "content_id, user_id, and body required".to_string(),
            ));
        }

        let created_at = if created_at <= 0 {
            now_timestamp()
        } else {
            created_at
        };

        let comment = Comment {
            id: Uuid::new_v4().simple().to_string(),
            content_id,
            user_id,
            body,
            created_at,
        };

        sqlx::query(
            r#"
            INSERT INTO comments (id, content_id, user_id, body, created_at)
            VALUES ($1, $2, $3, $4, $5)
            "#,
        )
        .bind(&comment.id)
        .bind(&comment.content_id)
        .bind(&comment.user_id)
        .bind(&comment.body)
        .bind(comment.created_at)
        .execute(&self.pool)
        .await
        .map_err(|err| EngagementError::Db(err.to_string()))?;

        Ok(comment)
    }

    pub async fn list_comments(&self, content_id: String) -> Result<Vec<Comment>, EngagementError> {
        let content_id = content_id.trim().to_string();
        if content_id.is_empty() {
            return Err(EngagementError::InvalidInput("content_id required".to_string()));
        }
        let rows = sqlx::query_as::<_, CommentRow>(
            r#"
            SELECT id, content_id, user_id, body, created_at
            FROM comments
            WHERE content_id = $1
            ORDER BY created_at ASC, id ASC
            "#,
        )
        .bind(&content_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|err| EngagementError::Db(err.to_string()))?;

        Ok(rows.into_iter().map(CommentRow::into_comment).collect())
    }

    pub async fn set_like(
        &self,
        content_id: String,
        user_id: String,
        liked: bool,
    ) -> Result<i64, EngagementError> {
        let content_id = content_id.trim().to_string();
        let user_id = user_id.trim().to_string();
        if content_id.is_empty() || user_id.is_empty() {
            return Err(EngagementError::InvalidInput(
                "content_id and user_id required".to_string(),
            ));
        }

        if liked {
            sqlx::query(
                r#"
                INSERT INTO likes (content_id, user_id)
                VALUES ($1, $2)
                ON CONFLICT (content_id, user_id) DO NOTHING
                "#,
            )
            .bind(&content_id)
            .bind(&user_id)
            .execute(&self.pool)
            .await
            .map_err(|err| EngagementError::Db(err.to_string()))?;
        } else {
            sqlx::query(
                r#"
                DELETE FROM likes
                WHERE content_id = $1 AND user_id = $2
                "#,
            )
            .bind(&content_id)
            .bind(&user_id)
            .execute(&self.pool)
            .await
            .map_err(|err| EngagementError::Db(err.to_string()))?;
        }

        let likes = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*) FROM likes WHERE content_id = $1
            "#,
        )
        .bind(&content_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|err| EngagementError::Db(err.to_string()))?;

        Ok(likes)
    }

    pub async fn summary(&self, content_id: String, user_id: Option<String>) -> Result<EngagementSummary, EngagementError> {
        let content_id = content_id.trim().to_string();
        if content_id.is_empty() {
            return Err(EngagementError::InvalidInput("content_id required".to_string()));
        }
        let user_id = user_id.and_then(|value| {
            let trimmed = value.trim().to_string();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed)
            }
        });

        let likes = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*) FROM likes WHERE content_id = $1
            "#,
        )
        .bind(&content_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|err| EngagementError::Db(err.to_string()))?;

        let comments = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*) FROM comments WHERE content_id = $1
            "#,
        )
        .bind(&content_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|err| EngagementError::Db(err.to_string()))?;

        let liked_by_user = if let Some(user_id) = user_id {
            sqlx::query_scalar::<_, bool>(
                r#"
                SELECT EXISTS(
                    SELECT 1 FROM likes WHERE content_id = $1 AND user_id = $2
                )
                "#,
            )
            .bind(&content_id)
            .bind(&user_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|err| EngagementError::Db(err.to_string()))?
        } else {
            false
        };

        Ok(EngagementSummary {
            likes,
            comments,
            liked_by_user,
        })
    }
}

#[derive(Debug, FromRow)]
struct CommentRow {
    id: String,
    content_id: String,
    user_id: String,
    body: String,
    created_at: i64,
}

impl CommentRow {
    fn into_comment(self) -> Comment {
        Comment {
            id: self.id,
            content_id: self.content_id,
            user_id: self.user_id,
            body: self.body,
            created_at: self.created_at,
        }
    }
}

fn now_timestamp() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}
