use crate::http;
use crate::http::extractor::VerifiedUser;
use crate::http::materials::{FileName, MaterialFile, Submission, helpers};
use crate::http::{ApiContext, Error};
use anyhow::anyhow;
use axum::Json;
use axum::extract::{Multipart, Path, State};
use chrono::{DateTime, Utc};
use sqlx::Row;
use uuid::Uuid;

pub async fn update_submission(
    Path(submission_id): Path<Uuid>,
    user: VerifiedUser,
    State(ctx): State<ApiContext>,
    mut multipart: Multipart,
) -> http::Result<Json<Submission>> {
    let row = sqlx::query("select user_id, status from submission where submission_id = $1")
        .bind(submission_id)
        .fetch_optional(&ctx.db)
        .await?
        .ok_or(Error::NotFound)?;

    let submission_user_id = row.get::<Uuid, _>("user_id");
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
    let mut has_updates = false; // если есть изменения, то и меняем статус ревью
    let mut keep_file_ids: Option<Vec<Uuid>> = None;
    let mut new_files: Vec<MaterialFile> = vec![];

    let mut tx = ctx.db.begin().await?;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|err| Error::Anyhow(anyhow!(err)))?
    {
        let name = field.name().ok_or_else(|| Error::BadRequest)?.to_string();

        match name.as_str() {
            "title" => {
                let value = field.text().await.map_err(|_| Error::BadRequest)?;
                if !value.is_empty() {
                    sqlx::query("update submission set title = $1, updated_at = $2 where submission_id = $3")
                        .bind(&value)
                        .bind(now)
                        .bind(submission_id)
                        .execute(&mut *tx)
                        .await?;
                    has_updates = true;
                }
            }
            "description" => {
                let value = field.text().await.map_err(|_| Error::BadRequest)?;
                sqlx::query("update submission set description = $1, updated_at = $2 where submission_id = $3")
                    .bind(&value)
                    .bind(now)
                    .bind(submission_id)
                    .execute(&mut *tx)
                    .await?;
                has_updates = true;
            }
            "courses" => {
                let value = field.text().await.map_err(|_| Error::BadRequest)?;
                let courses: Vec<String> =
                    serde_json::from_str(&value).map_err(|_| Error::BadRequest)?;
                if !courses.is_empty() {
                    sqlx::query("update submission set courses = $1, updated_at = $2 where submission_id = $3")
                        .bind(&courses)
                        .bind(now)
                        .bind(submission_id)
                        .execute(&mut *tx)
                        .await?;
                    has_updates = true;
                }
            }
            "subjects" => {
                let value = field.text().await.map_err(|_| Error::BadRequest)?;
                let subjects: Vec<String> =
                    serde_json::from_str(&value).map_err(|_| Error::BadRequest)?;
                if !subjects.is_empty() {
                    sqlx::query("update submission set subjects = $1, updated_at = $2 where submission_id = $3")
                        .bind(&subjects)
                        .bind(now)
                        .bind(submission_id)
                        .execute(&mut *tx)
                        .await?;
                    has_updates = true;
                }
            }
            "type" => {
                let value = field.text().await.map_err(|_| Error::BadRequest)?;
                if !value.is_empty() {
                    sqlx::query(
                        "update submission set type = $1, updated_at = $2 where submission_id = $3",
                    )
                    .bind(&value)
                    .bind(now)
                    .bind(submission_id)
                    .execute(&mut *tx)
                    .await?;
                    has_updates = true;
                }
            }
            "difficulty" => {
                let value = field.text().await.map_err(|_| Error::BadRequest)?;
                if !value.is_empty() {
                    sqlx::query("update submission set difficulty = $1, updated_at = $2 where submission_id = $3")
                        .bind(&value)
                        .bind(now)
                        .bind(submission_id)
                        .execute(&mut *tx)
                        .await?;
                    has_updates = true;
                }
            }
            "keepFileIds" => {
                let value = field.text().await.map_err(|_| Error::BadRequest)?;
                keep_file_ids = Some(serde_json::from_str(&value).map_err(|_| Error::BadRequest)?);
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

                let file_id = Uuid::new_v4();
                let file = MaterialFile {
                    id: file_id,
                    name: file_name.name.clone(),
                    extension: file_name.extension,
                    size_bytes: content.as_bytes().len() as u64,
                    mime_type,
                    url: None,
                };

                sqlx::query(
                    r#"insert into material_file (file_id, user_id, name, size_bytes, extension, mime_type, storage_key) 
                    values ($1, $2, $3, $4, $5, $6, $7)"#,
                )
                .bind(&file_id)
                .bind(&user.user_id)
                .bind(&file.name)
                .bind(file.size_bytes as i64)
                .bind(&file.extension)
                .bind(&file.mime_type)
                .bind(&file.url)
                .execute(&mut *tx)
                .await?;

                sqlx::query(
                    r#"insert into submission_file_rel (submission_id, file_id) values ($1, $2)"#,
                )
                .bind(submission_id)
                .bind(&file_id)
                .execute(&mut *tx)
                .await?;

                new_files.push(file);
                has_updates = true;
            }
            _ => {}
        }
    }

    if let Some(ids) = keep_file_ids {
        sqlx::query(
            r#"delete from submission_file_rel where submission_id = $1 and file_id not in (select unnest($2))"#,
        )
        .bind(submission_id)
        .bind(&ids)
        .execute(&mut *tx)
        .await?;
    }

    if has_updates {
        sqlx::query(
            "update submission set status = 'pending_review', submitted_at = coalesce(submitted_at, $1), updated_at = $1 where submission_id = $2",
        )
        .bind(now)
        .bind(submission_id)
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;

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
            id: f.get::<Uuid, _>("file_id"),
            name: f.get::<String, _>("name"),
            size_bytes: f.get::<i64, _>("size_bytes") as u64,
            extension: f.get::<String, _>("extension"),
            mime_type: f.get::<Option<String>, _>("mime_type"),
            url: None,
        })
        .collect();

    Ok(Json(Submission {
        id: updated_row.get::<Uuid, _>("submission_id"),
        files,
        status: updated_row.get::<String, _>("status"),
        moderator_comment: Some(
            updated_row
                .get::<Option<String>, _>("moderator_comment")
                .unwrap_or_default(),
        ),
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
