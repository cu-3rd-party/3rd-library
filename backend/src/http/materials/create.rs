use axum::extract::State;
use axum::Json;
use chrono::Utc;
use sqlx::Row;
use crate::http;
use crate::http::{ApiContext, Error};
use crate::http::extractor::AuthUser;
use crate::http::materials::{CreateSubmissionRequest, Material, Submission};

pub async fn create_submission(
    auth_user: AuthUser,
    State(ctx): State<ApiContext>,
    Json(req): Json<CreateSubmissionRequest>,
) -> http::Result<Json<Submission>> {
    // TODO: вынести в AuthUser и давать жвт содержащий инфу об стутсе верификации емэйла
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
    //

    let submission_id = uuid::Uuid::new_v4();
    let now = Utc::now();

    sqlx::query(
        r#"insert into submission (submission_id, user_id, title, description, courses, subjects, type, difficulty, status, submitted_at)
           values ($1, $2, $3, $4, $5, $6, $7, $8, 'pending_review', $9)"#
    )
    .bind(submission_id)
    .bind(auth_user.user_id)
    .bind(&req.title)
    .bind(&req.description)
    .bind(&req.courses)
    .bind(&req.subjects)
    .bind(&req.r#type)
    .bind(&req.difficulty)
    .bind(now)
    .execute(&ctx.db)
    .await?;

    for attachment in req.files {
        sqlx::query(r#"insert into material_file (user_id, name, size_bytes, extension, mime_type, storage_key)
        values ($1, $2, $3, $4, $5, $6)"#)
            .bind(auth_user.user_id)
            .bind()
        // sqlx::query(
        //     r#"insert into submission_file_rel (submission_id, file_id)
        //     values ($1, $2)"#
        // ).bind(submission_id)
    }

    Ok(Json(Submission {
        id: submission_id,
        material: Material {
            id: uuid::Uuid::nil(),
            author_id: auth_user.user_id,
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