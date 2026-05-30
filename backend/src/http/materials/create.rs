use crate::http;
use crate::http::extractor::VerifiedUser;
use crate::http::materials::{CreateSubmissionRequest, FileName, MaterialFile, Submission};
use crate::http::{ApiContext, Error};
use anyhow::anyhow;
use axum::Json;
use axum::extract::{Multipart, State};
use chrono::Utc;
use log::debug;
use uuid::Uuid;

#[utoipa::path(
    post,
    path = "/api/materials/submissions",
    tag = "materials",
    request_body(content = String, content_type = "multipart/form-data", description = "Multipart form with `data` JSON part and `files` parts"),
    responses(
        (status = 200, description = "Submission created", body = Submission),
        (status = 400, description = "Invalid multipart payload", body = crate::http::error::ApiError),
        (status = 401, description = "Verified authentication required", body = crate::http::error::ApiError),
        (status = 500, description = "Internal server error", body = crate::http::error::ApiError)
    ),
    security(("bearer_auth" = []))
)]
pub async fn create_submission(
    user: VerifiedUser,
    State(ctx): State<ApiContext>,
    mut multipart: Multipart,
) -> http::Result<Json<Submission>> {
    let submission_id = uuid::Uuid::new_v4();
    let now = Utc::now();
    let mut files = vec![];
    let mut has_data = false;
    let mut tx = ctx.db.begin().await?;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|err| Error::Anyhow(anyhow!(err)))?
    {
        let name = field.name().ok_or_else(|| Error::BadRequest)?.to_string();

        match name.as_str() {
            "data" => {
                let req: CreateSubmissionRequest =
                    serde_json::from_str(&field.text().await.map_err(|_| Error::BadRequest)?)
                        .map_err(|_| Error::BadRequest)?;

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
                    .execute(&mut *tx)
                    .await?;
                has_data = true;
            }
            "files" => {
                let file_name = FileName::new_valid(
                    field
                        .file_name()
                        .ok_or_else(|| Error::BadRequest)?
                        .to_string(),
                )
                .ok_or_else(|| Error::BadRequest)?;
                let mime_type = field
                    .content_type()
                    .map(|s| s.to_string())
                    .ok_or_else(|| Error::BadRequest)?;
                let content = field.bytes().await.map_err(|_| Error::BadRequest)?;
                let key = format!("/materials/{}", file_name.name);

                let result = ctx
                    .s3bucket
                    .put_object_with_content_type(&key, &content, &mime_type)
                    .await
                    .map_err(|err| Error::Anyhow(anyhow!(err)))?;
                debug!(
                    "Uploaded file to {} with status code {}",
                    &key,
                    result.status_code()
                );

                let file = MaterialFile {
                    id: Uuid::new_v4(),
                    name: file_name.name,
                    extension: file_name.extension,
                    size_bytes: content.len() as i64,
                    mime_type: Some(mime_type),
                    url: Some(key),
                };
                sqlx::query(
                    r#"insert into material_file (file_id, user_id, name, size_bytes, extension, mime_type, storage_key) 
                    values ($1, $2, $3, $4, $5, $6, $7)"#
                )
                    .bind(&file.id)
                    .bind(&user.user_id)
                    .bind(&file.name)
                    .bind(file.size_bytes) //srry bigint no more than i64
                    .bind(&file.extension)
                    .bind(&file.mime_type)
                    .bind(&file.url)
                    .execute(&mut *tx)
                    .await?;

                files.push(file);
            }
            _ => {
                return Err(Error::BadRequest);
            }
        }
    }

    if !has_data {
        return Err(Error::BadRequest);
    }

    for file in &files {
        sqlx::query(r#"insert into submission_file_rel (submission_id, file_id) values ($1, $2)"#)
            .bind(submission_id)
            .bind(file.id)
            .execute(&mut *tx)
            .await?;
    }

    tx.commit().await?;

    Ok(Json(Submission {
        id: submission_id,
        files,
        status: "pending_review".to_string(),
        created_at: now.format("%Y-%m-%dT%H:%M:%S%.fZ").to_string(),
        updated_at: now.format("%Y-%m-%dT%H:%M:%S%.fZ").to_string(),
        submitted_at: Some(now.format("%Y-%m-%dT%H:%M:%S%.fZ").to_string()),
        moderator_comment: None,
        reviewed_at: None,
        published_at: None,
    }))
}
