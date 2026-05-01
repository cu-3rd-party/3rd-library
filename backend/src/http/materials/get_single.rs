use crate::http::{ApiContext, Result};
use axum::Json;
use axum::extract::{Path, State};

use crate::http::error::Error;
use crate::http::extractor::AuthUser;
use chrono::{DateTime, Utc};
use sqlx::Row;

use super::models::*;

fn to_rfc3339(dt: DateTime<Utc>) -> String {
    dt.format("%Y-%m-%dT%H:%M:%S%.fZ").to_string()
}

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

    let pub_date: Option<String> = row
        .get::<Option<DateTime<Utc>>, _>("published_at")
        .map(to_rfc3339);

    Ok(Json(Submission {
        id: row.get::<uuid::Uuid, _>("submission_id"),
        material: Material {
            id: uuid::Uuid::nil(),
            author_id: submission_user_id,
            author_name: row.get::<Option<String>, _>("author_name"),
            title: row.get::<String, _>("title"),
            description: row
                .get::<Option<String>, _>("description")
                .unwrap_or_default(),
            courses: row
                .get::<Option<Vec<String>>, _>("courses")
                .unwrap_or_default(),
            subjects: row
                .get::<Option<Vec<String>>, _>("subjects")
                .unwrap_or_default(),
            r#type: row.get::<String, _>("type"),
            difficulty: row
                .get::<Option<String>, _>("difficulty")
                .unwrap_or_else(|| "none".to_string()),
            pub_date,
        },
        files,
        status: row.get::<String, _>("status"),
        moderator_comment: row
            .get::<Option<String>, _>("moderator_comment")
            .unwrap_or_default(),
        created_at: to_rfc3339(row.get::<DateTime<Utc>, _>("created_at")),
        updated_at: to_rfc3339(row.get::<DateTime<Utc>, _>("updated_at")),
        submitted_at: row
            .get::<Option<DateTime<Utc>>, _>("submitted_at")
            .map(to_rfc3339),
        reviewed_at: row
            .get::<Option<DateTime<Utc>>, _>("reviewed_at")
            .map(to_rfc3339),
        published_at: row
            .get::<Option<DateTime<Utc>>, _>("published_at")
            .map(to_rfc3339),
    }))
}

pub async fn update_submission(
    Path(submission_id): Path<uuid::Uuid>,
    auth_user: AuthUser,
    State(ctx): State<ApiContext>,
    Json(req): Json<UpdateSubmissionRequest>,
) -> Result<Json<Submission>> {
    let row = sqlx::query("select user_id, status from submission where submission_id = $1")
        .bind(submission_id)
        .fetch_optional(&ctx.db)
        .await?
        .ok_or(Error::NotFound)?;

    let submission_user_id = row.get::<uuid::Uuid, _>("user_id");
    if submission_user_id != auth_user.user_id {
        return Err(Error::Forbidden);
    }

    let submission_status = row.get::<String, _>("status");
    if submission_status != "draft" && submission_status != "rejected" {
        return Err(Error::forbidden(
            "invalid_status",
            "Can only update draft or rejected submissions",
        ));
    }

    let user_row = sqlx::query("select is_email_verified from web_user where user_id = $1")
        .bind(auth_user.user_id)
        .fetch_one(&ctx.db)
        .await?;

    if !user_row.get::<bool, _>("is_email_verified") {
        return Err(Error::forbidden(
            "email_not_verified",
            "Email is not verified",
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
        .map(to_rfc3339);

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
        created_at: to_rfc3339(updated_row.get::<DateTime<Utc>, _>("created_at")),
        updated_at: to_rfc3339(updated_row.get::<DateTime<Utc>, _>("updated_at")),
        submitted_at: updated_row
            .get::<Option<DateTime<Utc>>, _>("submitted_at")
            .map(to_rfc3339),
        reviewed_at: updated_row
            .get::<Option<DateTime<Utc>>, _>("reviewed_at")
            .map(to_rfc3339),
        published_at: updated_row
            .get::<Option<DateTime<Utc>>, _>("published_at")
            .map(to_rfc3339),
    }))
}
