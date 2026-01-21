use crate::http::ApiContext;
use crate::http::extractor::{AuthUser, MaybeAuthUser};
use crate::http::profiles::Profile;
use crate::http::types::Timestamptz;
use crate::http::{Error, Result};
use axum::extract::{Path, State};
use axum::routing::{delete, get};
use axum::{Json, Router};
use futures::TryStreamExt;
use serde::Serialize;
use time::OffsetDateTime;
use uuid::Uuid;

pub fn router() -> Router<ApiContext> {
    Router::new().route("/api/articles/{slug}/attachments", get(get_attachments))
    // .route(
    //     "/api/articles/{slug}/attachments/{attachment_id}",
    //     delete(todo!()),
    // )
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
