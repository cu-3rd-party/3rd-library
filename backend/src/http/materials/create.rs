use crate::http;
use crate::http::extractor::VerifiedUser;
use crate::http::materials::{CreateSubmissionRequest, FileName, MaterialFile, Submission};
use crate::http::{ApiContext, Error};
use anyhow::anyhow;
use axum::Json;
use axum::extract::{Multipart, State};
use chrono::Utc;
use uuid::Uuid;

pub async fn create_submission(
    user: VerifiedUser,
    State(ctx): State<ApiContext>,
    mut multipart: Multipart,
) -> http::Result<Json<Submission>> {
    let submission_id = uuid::Uuid::new_v4();
    let now = Utc::now();
    let mut files = vec![];

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
                    .execute(&ctx.db)
                    .await?;
            }
            "files" => {
                let file_name = FileName::new_valid(
                    field
                        .file_name()
                        .ok_or_else(|| Error::BadRequest)?
                        .to_string(),
                )
                .ok_or_else(|| Error::BadRequest)?;
                let mime_type = field.content_type().map(|s| s.to_string());
                let content = field.text().await.map_err(|_| Error::BadRequest)?;

                let file = MaterialFile {
                    id: Uuid::new_v4(),
                    name: file_name.name,
                    extension: file_name.extension,
                    size_bytes: content.as_bytes().len() as i64,
                    mime_type,
                    url: None, // TODO: Uploading to s3 storage
                };
                sqlx::query(
                    r#"insert into material_file (file_id, user_id, name, size_bytes, extension, mime_type, storage_key) 
                    values ($1, $2, $3, $4, $5, $6, $7)"#
                )
                    .bind(&file.id)
                    .bind(&user.user_id)
                    .bind(&file.name)
                    .bind(&file.extension)
                    .bind(file.size_bytes as i64) //srry bigint no more than i64
                    .bind(&file.mime_type)
                    .bind(&file.url)
                    .execute(&ctx.db)
                    .await?;

                files.push(file);
            }
            _ => {
                return Err(Error::BadRequest);
            }
        }
    }

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
