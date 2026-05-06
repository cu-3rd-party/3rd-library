// Placeholder - update functionality is in get_single.rs

use axum::extract::{Path, State};
use axum::Json;
use chrono::{DateTime, Utc};
use sqlx::Row;
use crate::http;
use crate::http::{ApiContext, Error};
use crate::http::extractor::{VerifiedUser};
use crate::http::materials::{helpers, Material, Submission, UpdateSubmissionRequest};

pub async fn update_submission(
    Path(submission_id): Path<uuid::Uuid>,
    user: VerifiedUser,
    State(ctx): State<ApiContext>,
    Json(req): Json<UpdateSubmissionRequest>,
) -> http::Result<Json<Submission>> {
    let row = sqlx::query("select user_id, status from submission where submission_id = $1")
        .bind(submission_id)
        .fetch_optional(&ctx.db)
        .await?
        .ok_or(Error::NotFound)?;

    let submission_user_id = row.get::<uuid::Uuid, _>("user_id");
    if submission_user_id != user.user_id {
        return Err(Error::Forbidden);
    }

    let submission_status = row.get::<String, _>("status");
    if submission_status != "draft" && submission_status != "rejected" {
        return Err(Error::forbidden(
            "invalid_status",
            "Can only update draft or rejected submissions",
        ));
    }

    let now = Utc::now();

    sqlx::query(
        r#"
            update submission 
            set title = coalesce($1, title),
                description = coalesce($2, description),
                courses = coalesce($3, courses),
                subjects = coalesce($4, subjects),
                type = coalesce($5, type),
                difficulty = coalesce($6, difficulty),
                status = 'pending_review',
                submitted_at = $7,
                updated_at = $7
            where submission_id = $8
        "#,
    )
    .bind(&req.title)
    .bind(&req.description)
    .bind(&req.courses)
    .bind(&req.subjects)
    .bind(&req.r#type)
    .bind(&req.difficulty)
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

    let pub_date: Option<String> = updated_row
        .get::<Option<DateTime<Utc>>, _>("published_at")
        .map(helpers::to_rfc3339);

    Ok(Json(Submission {
        id: updated_row.get::<uuid::Uuid, _>("submission_id"),
        material: Material {
            id: uuid::Uuid::nil(),
            author_id: updated_row.get::<uuid::Uuid, _>("user_id"),
            author_name: updated_row.get::<Option<String>, _>("author_name"),
            title: updated_row.get::<String, _>("title"),
            description: updated_row
                .get::<Option<String>, _>("description")
                .unwrap_or_default(),
            courses: updated_row
                .get::<Option<Vec<String>>, _>("courses")
                .unwrap_or_default(),
            subjects: updated_row
                .get::<Option<Vec<String>>, _>("subjects")
                .unwrap_or_default(),
            r#type: updated_row.get::<String, _>("type"),
            difficulty: updated_row
                .get::<Option<String>, _>("difficulty")
                .unwrap_or_else(|| "none".to_string()),
            pub_date,
        },
        files: vec![],
        status: updated_row.get::<String, _>("status"),
        moderator_comment: updated_row
            .get::<Option<String>, _>("moderator_comment")
            .unwrap_or_default(),
        created_at: helpers::to_rfc3339(updated_row.get::<DateTime<Utc>, _>("created_at")),
        updated_at: helpers::to_rfc3339(updated_row.get::<DateTime<Utc>, _>("updated_at")),
        submitted_at: updated_row
            .get::<Option<DateTime<Utc>>, _>("submitted_at")
            .map(helpers::to_rfc3339),
        reviewed_at: updated_row
            .get::<Option<DateTime<Utc>>, _>("reviewed_at")
            .map(helpers::to_rfc3339),
        published_at: updated_row
            .get::<Option<DateTime<Utc>>, _>("published_at")
            .map(helpers::to_rfc3339),
    }))
}