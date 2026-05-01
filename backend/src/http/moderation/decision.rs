use crate::http::{ApiContext, Result};
use axum::Json;
use axum::extract::{Path, State};
use chrono::{DateTime, Utc};
use sqlx::Row;

use crate::http::error::Error;
use crate::http::extractor::AuthUser;
use crate::http::materials::models::{Material, Submission};

use super::models::ModerationDecisionRequest;

fn to_rfc3339(dt: DateTime<Utc>) -> String {
    dt.format("%Y-%m-%dT%H:%M:%S%.fZ").to_string()
}

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
                    reviewed_at = $1,
                    published_at = $1,
                    updated_at = $1
                where submission_id = $2
            "#,
        )
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

        let now_rfc3339 = now.format("%Y-%m-%dT%H:%M:%S%.fZ").to_string();

        Ok(Json(Submission {
            id: updated_row.get::<uuid::Uuid, _>("submission_id"),
            material: Material {
                id: material_id,
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
                pub_date: Some(now_rfc3339.clone()),
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
                    reviewed_at = $2,
                    updated_at = $2
                where submission_id = $3
            "#,
        )
        .bind(&comment)
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
                pub_date: None,
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
    } else {
        Err(Error::bad_request(
            "invalid_action",
            "action must be 'approve' or 'reject'",
        ))
    }
}
