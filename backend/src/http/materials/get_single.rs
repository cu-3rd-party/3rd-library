use crate::http::{ApiContext, Result};
use axum::Json;
use axum::extract::{Path, State};

use super::models::*;
use crate::http::error::Error;
use crate::http::extractor::AuthUser;
use crate::http::materials::helpers;
use chrono::{DateTime, Utc};
use sqlx::Row;

#[utoipa::path(
    get,
    path = "/api/materials/submissions/{submissionId}",
    tag = "materials",
    params(("submissionId" = uuid::Uuid, Path, description = "Submission id")),
    responses(
        (status = 200, description = "Submission details", body = Submission),
        (status = 401, description = "Authentication required", body = crate::http::error::ApiError),
        (status = 403, description = "Access denied", body = crate::http::error::ApiError),
        (status = 404, description = "Submission not found", body = crate::http::error::ApiError),
        (status = 500, description = "Internal server error", body = crate::http::error::ApiError)
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_submission_by_id(
    Path(submission_id): Path<uuid::Uuid>,
    auth_user: AuthUser,
    State(ctx): State<ApiContext>,
) -> Result<Json<Submission>> {
    let row = sqlx::query(
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
            where s.submission_id = $1
        "#,
    )
    .bind(submission_id)
    .fetch_optional(&ctx.db)
    .await?
    .ok_or(Error::NotFound)?;

    let submission_user_id = row.get::<uuid::Uuid, _>("user_id");
    let is_owner = submission_user_id == auth_user.user_id;
    let user_row = sqlx::query("select roles from web_user where user_id = $1")
        .bind(auth_user.user_id)
        .fetch_one(&ctx.db)
        .await?;

    let is_moderator = user_row
        .get::<Option<Vec<String>>, _>("roles")
        .map(|r| r.contains(&"moderator".to_string()))
        .unwrap_or(false);

    if !is_owner && !is_moderator {
        return Err(Error::Forbidden);
    }

    let files_rows = sqlx::query(
        r#"
            select mf.file_id, mf.name, mf.size_bytes, mf.extension, mf.mime_type
            from material_file mf
            inner join submission_file_rel sfr on mf.file_id = sfr.file_id
            where sfr.submission_id = $1
        "#,
    )
    .bind(submission_id)
    .fetch_all(&ctx.db)
    .await?;

    let files: Vec<MaterialFile> = files_rows
        .into_iter()
        .map(|f| MaterialFile {
            id: f.get::<uuid::Uuid, _>("file_id"),
            name: f.get::<String, _>("name"),
            size_bytes: f.get::<i64, _>("size_bytes"),
            extension: f.get::<String, _>("extension"),
            mime_type: f.get::<Option<String>, _>("mime_type"),
            url: None,
        })
        .collect();

    Ok(Json(Submission {
        id: row.get::<uuid::Uuid, _>("submission_id"),
        files,
        status: row.get::<String, _>("status"),
        moderator_comment: Some(
            row.get::<Option<String>, _>("moderator_comment")
                .unwrap_or_default(),
        ),
        created_at: helpers::to_rfc3339(row.get::<DateTime<Utc>, _>("created_at")),
        updated_at: helpers::to_rfc3339(row.get::<DateTime<Utc>, _>("updated_at")),
        submitted_at: row
            .get::<Option<DateTime<Utc>>, _>("submitted_at")
            .map(helpers::to_rfc3339),
        reviewed_at: row
            .get::<Option<DateTime<Utc>>, _>("reviewed_at")
            .map(helpers::to_rfc3339),
        published_at: row
            .get::<Option<DateTime<Utc>>, _>("published_at")
            .map(helpers::to_rfc3339),
    }))
}
