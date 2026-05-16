use crate::http::{ApiContext, Result};
use axum::Json;
use axum::extract::{Path, State};
use chrono::Utc;
use sqlx::Row;

use crate::http::error::Error;
use crate::http::extractor::AuthUser;
use crate::http::materials::models::Submission;

use super::models::ModerationDecisionRequest;

#[utoipa::path(
    post,
    path = "/api/moderation/submissions/{submissionId}/decision",
    tag = "moderation",
    params(("submissionId" = uuid::Uuid, Path, description = "Submission id")),
    request_body = ModerationDecisionRequest,
    responses(
        (status = 200, description = "Moderation decision applied", body = Submission),
        (status = 400, description = "Invalid action or submission state", body = crate::http::error::ApiError),
        (status = 401, description = "Authentication required", body = crate::http::error::ApiError),
        (status = 403, description = "Moderator role required", body = crate::http::error::ApiError),
        (status = 404, description = "Submission not found", body = crate::http::error::ApiError),
        (status = 500, description = "Internal server error", body = crate::http::error::ApiError)
    ),
    security(("bearer_auth" = []))
)]
pub async fn moderation_decision(
    Path(submission_id): Path<uuid::Uuid>,
    auth_user: AuthUser,
    State(ctx): State<ApiContext>,
    Json(req): Json<ModerationDecisionRequest>,
) -> Result<Json<Submission>> {
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

    let submission_row = sqlx::query(
        "select submission_id, user_id, status from submission where submission_id = $1",
    )
    .bind(submission_id)
    .fetch_optional(&ctx.db)
    .await?
    .ok_or(Error::NotFound)?;

    if submission_row.get::<String, _>("status") != "pending_review" {
        return Err(Error::bad_request(
            "invalid_status",
            "Can only moderate pending_review submissions",
        ));
    }

    let now = Utc::now();

    if req.action == "approve" {
        let material_id = uuid::Uuid::new_v4();

        sqlx::query(
            r#"
                insert into material (material_id, user_id, title, description, courses, subjects, type, difficulty, published_at)
                select $1, user_id, title, description, courses, subjects, type, difficulty, $2
                from submission where submission_id = $3
            "#
        )
        .bind(material_id)
        .bind(now)
        .bind(submission_id)
        .execute(&ctx.db)
        .await?;

        sqlx::query(
            r#"
                update submission 
                set status = 'approved', 
                    moderator_comment = '', 
                    moderator_id = $1,
                    reviewed_at = $2,
                    published_at = $2,
                    updated_at = $2
                where submission_id = $3
            "#,
        )
        .bind(auth_user.user_id)
        .bind(now)
        .bind(submission_id)
        .execute(&ctx.db)
        .await?;

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

        for file in files_rows {
            sqlx::query("insert into material_file_rel (material_id, file_id) values ($1, $2)")
                .bind(material_id)
                .bind(file.get::<uuid::Uuid, _>("file_id"))
                .execute(&ctx.db)
                .await?;
        }

        let updated_row = sqlx::query(
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
        .fetch_one(&ctx.db)
        .await?;

        Ok(Json(updated_row.into()))
    } else if req.action == "reject" {
        let comment = req.moderator_comment.ok_or_else(|| {
            Error::bad_request(
                "moderator_comment_required",
                " moderator_comment is required for reject",
            )
        })?;

        sqlx::query(
            r#"
                update submission 
                set status = 'rejected', 
                    moderator_comment = $1, 
                    moderator_id = $2,
                    reviewed_at = $3,
                    updated_at = $3
                where submission_id = $4
            "#,
        )
        .bind(&comment)
        .bind(auth_user.user_id)
        .bind(now)
        .bind(submission_id)
        .execute(&ctx.db)
        .await?;

        let updated_row = sqlx::query(
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
        .fetch_one(&ctx.db)
        .await?;

        Ok(Json(updated_row.into()))
    } else {
        Err(Error::bad_request(
            "invalid_action",
            "action must be 'approve' or 'reject'",
        ))
    }
}
