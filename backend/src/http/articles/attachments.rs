use crate::http::ApiContext;
use crate::http::extractor::{AuthUser, MaybeAuthUser};
use crate::http::types::Timestamptz;
use crate::http::{Error, Result};
use crate::metrics;
use anyhow::Context;
use axum::extract::{Multipart, Path, State};
use axum::http::{HeaderMap, HeaderValue, header};
use axum::routing::get;
use axum::{Json, Router};
use futures::TryStreamExt;
use serde::Serialize;
use std::path::Path as StdPath;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::constants::UPLOAD_DIR;

pub fn router() -> Router<ApiContext> {
    Router::new()
        .route(
            "/api/articles/{slug}/attachments",
            get(get_attachments).post(create_attachment),
        )
        .route(
            "/api/articles/{slug}/attachments/{attachment_id}",
            get(get_attachment).delete(delete_attachment),
        )
}

struct AttachmentFromQuery {
    attachment_id: Uuid,
    file_name: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<AttachmentFromQuery> for Attachment {
    fn from(obj: AttachmentFromQuery) -> Self {
        Attachment {
            attachment_id: obj.attachment_id,
            file_name: obj.file_name,
            created_at: Timestamptz(obj.created_at),
            updated_at: Timestamptz(obj.updated_at),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Attachment {
    attachment_id: Uuid,
    file_name: String,
    created_at: Timestamptz,
    updated_at: Timestamptz,
}

#[derive(Serialize)]
struct MultipleAttachments {
    attachments: Vec<Attachment>,
}

#[derive(Serialize)]
struct AttachmentBody {
    attachment: Attachment,
}

async fn get_attachments(
    _maybe_auth_user: MaybeAuthUser,
    State(ctx): State<ApiContext>,
    Path(slug): Path<String>,
) -> Result<Json<MultipleAttachments>> {
    metrics::observe_db_query();
    let article_id: Option<uuid::Uuid> = sqlx::query_scalar("select article_id from article where slug = $1", slug)
        .fetch_optional(&ctx.db)
        .await?
        .ok_or(Error::NotFound)?;

    metrics::observe_db_query();
    let rows = sqlx::query(
        r#"
			select
				attachment_id,
				file_name,
				created_at,
				updated_at
			from attachment
			where article_id = $1
			order by created_at
		"#,
        article_id
    )
    .fetch(&ctx.db)
    .map_ok(|row| {
        AttachmentFromQuery {
            attachment_id: row.get::<uuid::Uuid, _>("attachment_id"),
            file_name: row.get::<String, _>("file_name"),
            created_at: row.get::<DateTime<Utc>, _>("created_at"),
            updated_at: row.get::<DateTime<Utc>, _>("updated_at"),
        }.into()
    })
    .try_collect()
    .await?;

    Ok(Json(MultipleAttachments { attachments: rows }))
}

async fn get_attachment(
    _maybe_auth_user: MaybeAuthUser,
    State(ctx): State<ApiContext>,
    Path((slug, attachment_id)): Path<(String, Uuid)>,
) -> Result<(HeaderMap, Vec<u8>)> {
    metrics::observe_db_query();
    let row = sqlx::query(
        r#"
            select attachment.file_path, attachment.file_name
            from attachment
            inner join article using (article_id)
            where attachment_id = $1 and slug = $2
        "#,
        attachment_id,
        slug
    )
    .fetch_optional(&ctx.db)
    .await?
    .ok_or(Error::NotFound)?;

    let file_path = row.get::<String, _>("file_path");
    let file_name = sanitize_filename(&row.get::<String, _>("file_name"));

    let read_result = tokio::task::spawn_blocking(move || std::fs::read(&file_path)).await;
    let file_bytes = match read_result {
        Ok(Ok(bytes)) => bytes,
        Ok(Err(err)) if err.kind() == std::io::ErrorKind::NotFound => return Err(Error::NotFound),
        Ok(Err(err)) => {
            return Err(anyhow::anyhow!(err)
                .context("failed to read attachment file")
                .into());
        }
        Err(err) => {
            return Err(anyhow::Error::from(err)
                .context("failed to join attachment read task")
                .into());
        }
    };

    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/octet-stream"),
    );
    let disposition = format!("attachment; filename=\"{}\"", file_name.replace('"', "_"));
    headers.insert(
        header::CONTENT_DISPOSITION,
        HeaderValue::from_str(&disposition)
            .context("failed to build content-disposition header")?,
    );

    Ok((headers, file_bytes))
}

async fn create_attachment(
    auth_user: AuthUser,
    State(ctx): State<ApiContext>,
    Path(slug): Path<String>,
    mut multipart: Multipart,
) -> Result<Json<AttachmentBody>> {
    let mut raw_file_name = None;
    let mut file_bytes = None;

    while let Some(field) = multipart
        .next_field()
        .await
        .context("failed to read multipart field")?
    {
        if file_bytes.is_some() {
            continue;
        }

        let is_file_field = matches!(field.name(), Some("file")) || raw_file_name.is_none();
        if !is_file_field {
            continue;
        }

        raw_file_name = Some(
            field
                .file_name()
                .map(|name| name.to_string())
                .unwrap_or_else(|| "attachment".to_string()),
        );
        file_bytes = Some(
            field
                .bytes()
                .await
                .context("failed to read attachment file")?
                .to_vec(),
        );
    }

    let raw_file_name =
        raw_file_name.ok_or_else(|| Error::unprocessable_entity([("file", "missing")]))?;
    let file_bytes =
        file_bytes.ok_or_else(|| Error::unprocessable_entity([("file", "missing")]))?;

    let file_name = sanitize_filename(&raw_file_name);
    let attachment_id = Uuid::new_v4();
    let stored_name = match StdPath::new(&file_name)
        .extension()
        .and_then(|ext| ext.to_str())
    {
        Some(ext) => format!("{}.{}", attachment_id, ext),
        None => attachment_id.to_string(),
    };
    let file_path = format!("{}/{}", UPLOAD_DIR, stored_name);

    metrics::observe_db_query();
    let row = sqlx::query(
        r#"
            with article_match as (
                select article_id
                from article
                where slug = $1 and user_id = $2
            ),
            inserted as (
                insert into attachment (attachment_id, article_id, file_name, file_path)
                select $3, article_id, $4, $5
                from article_match
                returning attachment_id, file_name, created_at, updated_at
            )
            select
                exists(select 1 from article where slug = $1) as existed,
                exists(select 1 from article_match) as authorized,
                (select attachment_id from inserted) as attachment_id,
                (select file_name from inserted) as file_name,
                (select created_at from inserted) as created_at,
                (select updated_at from inserted) as updated_at
        "#,
        slug,
        auth_user.user_id,
        attachment_id,
        file_name,
        &file_path,
    )
    .fetch_one(&ctx.db)
    .await?;

    if !row.get::<bool, _>("existed") {
        return Err(Error::NotFound);
    }

    if !row.get::<bool, _>("authorized") {
        return Err(Error::Forbidden);
    }

    let attachment = Attachment {
        attachment_id: row.get::<Option<uuid::Uuid>, _>("attachment_id")
            .ok_or_else(|| anyhow::anyhow!("attachment insert failed"))?,
        file_name: row.get::<Option<String>, _>("file_name")
            .ok_or_else(|| anyhow::anyhow!("attachment insert failed"))?,
        created_at: Timestamptz(
            row.get::<Option<DateTime<Utc>>, _>("created_at")
                .ok_or_else(|| anyhow::anyhow!("attachment insert failed"))?,
        ),
        updated_at: Timestamptz(
            row.get::<Option<DateTime<Utc>>, _>("updated_at")
                .ok_or_else(|| anyhow::anyhow!("attachment insert failed"))?,
        ),
    };

    let write_path = file_path.clone();
    let write_bytes = file_bytes;
    let write_result = tokio::task::spawn_blocking(move || {
        std::fs::create_dir_all(UPLOAD_DIR)?;
        std::fs::write(&write_path, &write_bytes)?;
        Ok::<(), std::io::Error>(())
    })
    .await;

    match write_result {
        Ok(Ok(())) => {
            metrics::record_attachment_created();
        }
        Ok(Err(err)) => {
            metrics::observe_db_query();
            let _ = sqlx::query(
                "delete from attachment where attachment_id = $1",
                attachment_id
            )
            .execute(&ctx.db)
            .await;
            return Err(anyhow::anyhow!(err)
                .context("failed to write attachment file")
                .into());
        }
        Err(err) => {
            metrics::observe_db_query();
            let _ = sqlx::query(
                "delete from attachment where attachment_id = $1",
                attachment_id
            )
            .execute(&ctx.db)
            .await;
            return Err(anyhow::Error::from(err)
                .context("failed to join attachment write task")
                .into());
        }
    }

    Ok(Json(AttachmentBody { attachment }))
}

fn sanitize_filename(name: &str) -> String {
    StdPath::new(name)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("attachment")
        .to_string()
}

#[test]
fn test_sanitize_filename() {
    assert_eq!(sanitize_filename("../../text.txt"), "text.txt");
    assert_eq!(sanitize_filename("./test/text.txt"), "text.txt");
    assert_eq!(sanitize_filename("/dev/null"), "null");
}

async fn delete_attachment(
    auth_user: AuthUser,
    State(ctx): State<ApiContext>,
    Path((slug, attachment_id)): Path<(String, Uuid)>,
) -> Result<()> {
    metrics::observe_db_query();
    let row = sqlx::query(
        r#"
            with deleted_attachment as (
                delete from attachment
                using article
                where
                    attachment.attachment_id = $1
                    and attachment.article_id = article.article_id
                    and article.slug = $2
                    and article.user_id = $3
                returning attachment.file_path
            )
            select
                exists(
                    select 1 from attachment
                    inner join article using (article_id)
                    where attachment_id = $1 and slug = $2
                ) as existed,
                exists(select 1 from deleted_attachment) as deleted,
                (select file_path from deleted_attachment) as file_path
        "#,
        attachment_id,
        slug,
        auth_user.user_id,
    )
    .fetch_one(&ctx.db)
    .await?;

    if row.get::<bool, _>("deleted") {
        if let Some(file_path) = row.get::<Option<String>, _>("file_path") {
            let path_for_error = file_path.clone();
            tokio::task::spawn_blocking(move || std::fs::remove_file(&file_path))
                .await
                .context("failed to join attachment cleanup task")?
                .with_context(|| format!("failed to delete attachment file: {}", path_for_error))?;
        }
        Ok(())
    } else if row.get::<bool, _>("existed") {
        Err(Error::Forbidden)
    } else {
        Err(Error::NotFound)
    }
}
