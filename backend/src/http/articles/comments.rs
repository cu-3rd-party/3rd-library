use crate::http::ApiContext;
use crate::http::extractor::{AuthUser, MaybeAuthUser};
use crate::http::profiles::Profile;
use crate::http::types::Timestamptz;
use crate::http::{Error, Result};
use crate::metrics;
use axum::extract::{Path, State};
use axum::routing::{delete, get};
use axum::{Json, Router};
use futures::TryStreamExt;
use time::OffsetDateTime;

pub fn router() -> Router<ApiContext> {
    Router::new()
        .route(
            "/api/articles/{slug}/comments",
            get(get_article_comments).post(add_comment),
        )
        .route(
            "/api/articles/{slug}/comments/{comment_id}",
            delete(delete_comment),
        )
}

#[derive(serde::Deserialize, serde::Serialize)]
struct CommentBody<T = Comment> {
    comment: T,
}

#[derive(serde::Serialize)]
struct MultipleCommentsBody {
    comments: Vec<Comment>,
}

#[derive(serde::Deserialize)]
struct AddComment {
    body: String,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct Comment {
    id: i64,
    created_at: Timestamptz,
    updated_at: Timestamptz,
    body: String,
    author: Profile,
}

struct CommentFromQuery {
    comment_id: i64,
    created_at: OffsetDateTime,
    updated_at: OffsetDateTime,
    body: String,
    author_username: String,
    author_bio: String,
    author_image: Option<uuid::Uuid>,
    following_author: bool,
}

impl CommentFromQuery {
    fn into_comment(self) -> Comment {
        Comment {
            id: self.comment_id,
            created_at: Timestamptz(self.created_at),
            updated_at: Timestamptz(self.updated_at),
            body: self.body,
            author: Profile {
                username: self.author_username,
                bio: self.author_bio,
                image: self.author_image,
                following: self.following_author,
            },
        }
    }
}

async fn get_article_comments(
    maybe_auth_user: MaybeAuthUser,
    State(ctx): State<ApiContext>,
    Path(slug): Path<String>,
) -> Result<Json<MultipleCommentsBody>> {
    metrics::observe_db_query();
    let article_id: Option<uuid::Uuid> = sqlx::query_scalar("select article_id from article where slug = $1", slug)
        .fetch_optional(&ctx.db)
        .await?
        .ok_or(Error::NotFound)?;

    metrics::observe_db_query();
    let rows = sqlx::query(
        r#"
            select
                comment_id,
                comment.created_at,
                comment.updated_at,
                comment.body,
                author.username as author_username,
                author.bio as author_bio,
                author.pfp_id as author_image,
                exists(select 1 from follow where followed_user_id = author.user_id and following_user_id = $1) as following_author
            from article_comment comment
            inner join "user" author using (user_id)
            where article_id = $2
            order by created_at
        "#,
        maybe_auth_user.user_id(),
        article_id
    )
        .fetch(&ctx.db)
        .map_ok(|row| {
            CommentFromQuery {
                comment_id: row.get::<i64, _>("comment_id"),
                created_at: row.get::<time::PrimitiveDateTime, _>("created_at").into(),
                updated_at: row.get::<time::PrimitiveDateTime, _>("updated_at").into(),
                body: row.get::<String, _>("body"),
                author_username: row.get::<String, _>("author_username"),
                author_bio: row.get::<Option<String>, _>("author_bio").unwrap_or_default(),
                author_image: row.get::<Option<uuid::Uuid>, _>("author_image"),
                following_author: row.get::<bool, _>("following_author"),
            }.into_comment()
        })
        .try_collect()
        .await?;

    Ok(Json(MultipleCommentsBody { comments: rows }))
}

async fn add_comment(
    auth_user: AuthUser,
    State(ctx): State<ApiContext>,
    Path(slug): Path<String>,
    req: Json<CommentBody<AddComment>>,
) -> Result<Json<CommentBody>> {
    metrics::observe_db_query();
    let row = sqlx::query(
        r#"
            with inserted_comment as (
                insert into article_comment(article_id, user_id, body)
                select article_id, $1, $2
                from article
                where slug = $3
                returning comment_id, created_at, updated_at, body
            )
            select
                comment_id,
                comment.created_at,
                comment.updated_at,
                body,
                author.username as author_username,
                author.bio as author_bio,
                author.pfp_id as author_image,
                false as following_author
            from inserted_comment comment
            inner join "user" author on user_id = $1
        "#,
        auth_user.user_id,
        req.comment.body,
        slug
    )
    .fetch_optional(&ctx.db)
    .await?
    .ok_or(Error::NotFound)?;

    let comment = CommentFromQuery {
        comment_id: row.get::<i64, _>("comment_id"),
        created_at: row.get::<time::PrimitiveDateTime, _>("created_at").into(),
        updated_at: row.get::<time::PrimitiveDateTime, _>("updated_at").into(),
        body: row.get::<String, _>("body"),
        author_username: row.get::<String, _>("author_username"),
        author_bio: row.get::<Option<String>, _>("author_bio").unwrap_or_default(),
        author_image: row.get::<Option<uuid::Uuid>, _>("author_image"),
        following_author: row.get::<bool, _>("following_author"),
    };

    metrics::record_comment_created();

    Ok(Json(CommentBody { comment: comment.into_comment() }))
}

async fn delete_comment(
    auth_user: AuthUser,
    State(ctx): State<ApiContext>,
    Path((slug, comment_id)): Path<(String, i64)>,
) -> Result<()> {
    metrics::observe_db_query();
    let row = sqlx::query(
        r#"
            with deleted_comment as (
                delete from article_comment
                where 
                    comment_id = $1
                    and article_id in (select article_id from article where slug = $2)
                    and user_id = $3
                returning 1 
            )
            select 
                exists(
                    select 1 from article_comment
                    inner join article using (article_id)
                    where comment_id = $1 and slug = $2
                ) as existed,
                exists(select 1 from deleted_comment) as deleted
        "#,
        comment_id,
        slug,
        auth_user.user_id
    )
    .fetch_one(&ctx.db)
    .await?;

    if row.get::<bool, _>("deleted") {
        Ok(())
    } else if row.get::<bool, _>("existed") {
        Err(Error::Forbidden)
    } else {
        Err(Error::NotFound)
    }
}
