use crate::http::ApiContext;
use crate::http::extractor::{AuthUser, MaybeAuthUser};
use crate::http::types::Timestamptz;
use crate::http::{Error, Result};
use anyhow::Context;
use axum::extract::{Path, State};
use axum::routing::{delete, get};
use axum::{Json, Router};
use futures::TryStreamExt;
use serde::Serialize;
use time::OffsetDateTime;
use uuid::Uuid;

pub fn router() -> Router<ApiContext> {
    Router::new()
        .route("/api/articles/{slug}/attachments", get(get_attachments))
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

async fn get_attachments(
    maybe_auth_user: MaybeAuthUser,
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
