use crate::http;
use crate::http::extractor::{AuthUser, VerifiedUser};
use crate::http::materials::{CreateSubmissionRequest, Material, Submission};
use crate::http::{ApiContext, Error};
use axum::Json;
use axum::extract::State;
use chrono::Utc;
use sqlx::Row;

pub async fn create_submission(
    user: VerifiedUser,
    State(ctx): State<ApiContext>,
    Json(req): Json<CreateSubmissionRequest>,
) -> http::Result<Json<Submission>> {
    let submission_id = uuid::Uuid::new_v4();
    let now = Utc::now();

    sqlx::query(
        r#"insert into submission (submission_id, user_id, title, description, courses, subjects, type, difficulty, status, submitted_at)
           values ($1, $2, $3, $4, $5, $6, $7, $8, 'pending_review', $9)"#
    )
    .bind(submission_id)
    .bind(user.user_id)
    .bind(&req.title)
    .bind(&req.description)
    .bind(&req.courses)
    .bind(&req.subjects)
    .bind(&req.r#type)
    .bind(&req.difficulty)
    .bind(now)
    .execute(&ctx.db)
    .await?;

    Ok(Json(Submission {
        id: submission_id,
        material: Material {
            id: uuid::Uuid::nil(),
            author_id: user.user_id,
            author_name: None,
            title: req.title,
            description: req.description.unwrap_or_default(),
            courses: req.courses,
            subjects: req.subjects,
            r#type: req.r#type,
            difficulty: req.difficulty,
            pub_date: None,
        },
        files: vec![],
        status: "pending_review".to_string(),
        moderator_comment: String::new(),
        created_at: now.format("%Y-%m-%dT%H:%M:%S%.fZ").to_string(),
        updated_at: now.format("%Y-%m-%dT%H:%M:%S%.fZ").to_string(),
        submitted_at: Some(now.format("%Y-%m-%dT%H:%M:%S%.fZ").to_string()),
        reviewed_at: None,
        published_at: None,
    }))
}
