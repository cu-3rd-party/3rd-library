use crate::http::{ApiContext, Result};
use axum::Json;
use axum::extract::{Query, State};
use serde::Deserialize;
use sqlx::Row;

use crate::http::error::Error;
use crate::http::extractor::AuthUser;
use crate::http::materials::models::Submission;

use super::models::*;

#[derive(Deserialize)]
pub struct ModerationQuery {
    pub status: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

pub async fn list_moderation_submissions(
    auth_user: AuthUser,
    Query(query): Query<ModerationQuery>,
    State(ctx): State<ApiContext>,
) -> Result<Json<PaginatedModerationResponse>> {
    let user_row = sqlx::query("select roles from web_user where user_id = $1")
        .bind(auth_user.user_id)
        .fetch_one(&ctx.db)
        .await?;

    let is_moderator = user_row
        .get::<Option<Vec<String>>, _>("roles")
        .map(|r| r.contains(&"moderator".to_string()))
        .unwrap_or(false);
    if !is_moderator {
        return Err(Error::Forbidden);
    }

    let page = query.page.unwrap_or(1).max(1);
    let limit = query.limit.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * limit;

    let status_filter = query.status.unwrap_or_else(|| "pending_review".to_string());

    let counters_row = sqlx::query(
        r#"
            select 
                count(*)::int8 as total,
                count(*) filter (where status = 'draft')::int8 as draft,
                count(*) filter (where status = 'pending_review')::int8 as pending_review,
                count(*) filter (where status = 'rejected')::int8 as rejected,
                count(*) filter (where status = 'approved')::int8 as approved
            from submission
        "#,
    )
    .fetch_one(&ctx.db)
    .await?;

    let total: i64 = counters_row.get("total");

    let limit_plus_one = limit + 1;
    let rows = sqlx::query(
        r#"
            select
                s.submission_id,
                s.user_id,
                s.title,
                s.description,
                s.courses,
                s.subjects,
                s.type,
                s.difficulty,
                s.status,
                s.moderator_comment,
                s.created_at,
                s.updated_at,
                s.submitted_at,
                s.reviewed_at,
                s.published_at,
                u.name as author_name
            from submission s
            inner join web_user u on s.user_id = u.user_id
            where ($1::text is null or s.status = $1)
            order by s.created_at asc
            limit $2 offset $3
        "#,
    )
    .bind(&status_filter)
    .bind(limit_plus_one)
    .bind(offset)
    .fetch_all(&ctx.db)
    .await?;

    let items: Vec<Submission> = rows
        .into_iter()
        .take(limit as usize)
        .map(|s| s.into())
        .collect();

    Ok(Json(PaginatedModerationResponse {
        items,
        page,
        limit,
        total,
        counters: SubmissionStatusCounters {
            all: total,
            draft: counters_row.get("draft"),
            pending_review: counters_row.get("pending_review"),
            rejected: counters_row.get("rejected"),
            approved: counters_row.get("approved"),
        },
    }))
}
