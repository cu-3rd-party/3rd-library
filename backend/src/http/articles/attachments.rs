use crate::http::ApiContext;
use crate::http::extractor::{AuthUser, MaybeAuthUser};
use crate::http::types::Timestamptz;
use crate::http::{Error, Result};
use anyhow::Context;
use axum::extract::{Multipart, Path, State};
use axum::routing::{delete, get};
use axum::{Json, Router};
use futures::TryStreamExt;
use serde::Serialize;
use std::path::Path as StdPath;
use time::OffsetDateTime;
use uuid::Uuid;

const UPLOAD_DIR: &str = "/var/app/upload";

pub fn router() -> Router<ApiContext> {
    Router::new()
        .route(
            "/api/articles/{slug}/attachments",
            get(get_attachments).post(create_attachment),
        )
        .route(
            "/api/articles/{slug}/attachments/{attachment_id}",
            delete(delete_attachment),
        )
}

struct AttachmentFromQuery {
    attachment_id: Uuid,
    file_name: String,
    created_at: OffsetDateTime,
    updated_at: OffsetDateTime,
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
    let article_id = sqlx::query_scalar!("select article_id from article where slug = $1", slug)
        .fetch_optional(&ctx.db)
        .await?
        .ok_or(Error::NotFound)?;

    let attachments: Vec<Attachment> = sqlx::query_as!(
        AttachmentFromQuery,
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
    .map_ok(|o| o.into())
    .try_collect()
    .await?;

    Ok(Json(MultipleAttachments { attachments }))
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

    let result = sqlx::query!(
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
                exists(select 1 from article where slug = $1) "existed!",
                exists(select 1 from article_match) "authorized!",
                (select attachment_id from inserted) "attachment_id?",
                (select file_name from inserted) "file_name?",
                (select created_at from inserted) "created_at?",
                (select updated_at from inserted) "updated_at?"
        "#,
        slug,
        auth_user.user_id,
        attachment_id,
        file_name,
        &file_path,
    )
    .fetch_one(&ctx.db)
    .await?;

    if !result.existed {
        return Err(Error::NotFound);
    }

    if !result.authorized {
        return Err(Error::Forbidden);
    }

    let attachment = Attachment {
        attachment_id: result
            .attachment_id
            .ok_or_else(|| anyhow::anyhow!("attachment insert failed"))?,
        file_name: result
            .file_name
            .ok_or_else(|| anyhow::anyhow!("attachment insert failed"))?,
        created_at: Timestamptz(
            result
                .created_at
                .ok_or_else(|| anyhow::anyhow!("attachment insert failed"))?,
        ),
        updated_at: Timestamptz(
            result
                .updated_at
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
        Ok(Ok(())) => {}
        Ok(Err(err)) => {
            let _ = sqlx::query!(
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
            let _ = sqlx::query!(
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

async fn delete_attachment(
    auth_user: AuthUser,
    State(ctx): State<ApiContext>,
    Path((slug, attachment_id)): Path<(String, Uuid)>,
) -> Result<()> {
    let result = sqlx::query!(
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
                ) "existed!",
                exists(select 1 from deleted_attachment) "deleted!",
                (select file_path from deleted_attachment) "file_path?"
        "#,
        attachment_id,
        slug,
        auth_user.user_id,
    )
    .fetch_one(&ctx.db)
    .await?;

    if result.deleted {
        if let Some(file_path) = result.file_path {
            let path_for_error = file_path.clone();
            tokio::task::spawn_blocking(move || std::fs::remove_file(&file_path))
                .await
                .context("failed to join attachment cleanup task")?
                .with_context(|| format!("failed to delete attachment file: {}", path_for_error))?;
        }
        Ok(())
    } else if result.existed {
        Err(Error::Forbidden)
    } else {
        Err(Error::NotFound)
    }
}
