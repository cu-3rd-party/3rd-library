use axum::extract::{Path, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use itertools::Itertools;
use sqlx::{Executor, Postgres};
use uuid::Uuid;

use crate::http::extractor::{AuthUser, MaybeAuthUser};
use crate::http::profiles::Profile;
use crate::http::types::Timestamptz;
use crate::http::{ApiContext, Error, Result, ResultExt};
use crate::metrics;

mod attachments;
mod comments;
mod listing;

pub fn router() -> Router<ApiContext> {
    Router::new()
        .route(
            "/api/articles",
            post(create_article).get(listing::list_articles),
        )
        .route("/api/articles/feed", get(listing::feed_articles))
        .route(
            "/api/articles/{slug}",
            get(get_article).put(update_article).delete(delete_article),
        )
        .route(
            "/api/articles/{slug}/favorite",
            post(favorite_article).delete(unfavorite_article),
        )
        .route("/api/tags", get(get_tags))
        .merge(comments::router())
        .merge(attachments::router())
}

#[derive(serde::Deserialize, serde::Serialize)]
struct ArticleBody<T = Article> {
    article: T,
}

#[derive(serde::Serialize)]
struct TagsBody {
    tags: Vec<String>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateArticle {
    title: String,
    description: String,
    body: String,
    tag_list: Vec<String>,
}

#[derive(serde::Deserialize)]
struct UpdateArticle {
    title: Option<String>,
    description: Option<String>,
    body: Option<String>,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct Article {
    slug: String,
    title: String,
    description: String,
    body: String,
    tag_list: Vec<String>,
    created_at: Timestamptz,
    updated_at: Timestamptz,
    favorited: bool,
    favorites_count: i64,
    author: Profile,
}

struct ArticleFromQuery {
    slug: String,
    title: String,
    description: String,
    body: String,
    tag_list: Vec<String>,
    created_at: Timestamptz,
    updated_at: Timestamptz,
    favorited: bool,
    favorites_count: i64,
    author_username: String,
    author_bio: String,
    author_image: Option<uuid::Uuid>,
    following_author: bool,
}

impl ArticleFromQuery {
    fn into_article(self) -> Article {
        Article {
            slug: self.slug,
            title: self.title,
            description: self.description,
            body: self.body,
            tag_list: self.tag_list,
            created_at: self.created_at,
            updated_at: self.updated_at,
            favorited: self.favorited,
            favorites_count: self.favorites_count,
            author: Profile {
                username: self.author_username,
                bio: self.author_bio,
                image: self.author_image,
                following: self.following_author,
            },
        }
    }
}

async fn create_article(
    auth_user: AuthUser,
    State(ctx): State<ApiContext>,
    Json(mut req): Json<ArticleBody<CreateArticle>>,
) -> Result<Json<ArticleBody>> {
    let slug = slugify(&req.article.title);

    req.article.tag_list.sort();

    metrics::observe_db_query();
    let row = sqlx::query(
        r#"
            with inserted_article as (
                insert into article (user_id, slug, title, description, body, tag_list)
                values ($1, $2, $3, $4, $5, $6)
                returning 
                    slug, 
                    title, 
                    description, 
                    body, 
                    tag_list, 
                    created_at, 
                    updated_at
            )
            select 
                inserted_article.*,
                false as favorited,
                0::int8 as favorites_count,
                username as author_username,
                bio as author_bio,
                pfp_id as author_image,
                false as following_author
            from inserted_article
            inner join "user" on user_id = $1
        "#,
        auth_user.user_id,
        slug,
        req.article.title,
        req.article.description,
        req.article.body,
        &req.article.tag_list[..]
    )
    .fetch_one(&ctx.db)
    .await
    .on_constraint("article_slug_key", |_| {
        Error::unprocessable_entity([("slug", format!("duplicate article slug: {}", slug))])
    })?;

    let article = ArticleFromQuery {
        slug: row.get::<String, _>("slug"),
        title: row.get::<String, _>("title"),
        description: row.get::<String, _>("description"),
        body: row.get::<String, _>("body"),
        tag_list: row.get::<Vec<String>, _>("tag_list"),
        created_at: row.get::<Timestamptz, _>("created_at"),
        updated_at: row.get::<Timestamptz, _>("updated_at"),
        favorited: row.get::<bool, _>("favorited"),
        favorites_count: row.get::<i64, _>("favorites_count"),
        author_username: row.get::<String, _>("author_username"),
        author_bio: row.get::<Option<String>, _>("author_bio").unwrap_or_default(),
        author_image: row.get::<Option<uuid::Uuid>, _>("author_image"),
        following_author: row.get::<bool, _>("following_author"),
    };

    metrics::record_article_created();

    Ok(Json(ArticleBody {
        article: article.into_article(),
    }))
}

async fn update_article(
    auth_user: AuthUser,
    State(ctx): State<ApiContext>,
    Path(slug): Path<String>,
    Json(req): Json<ArticleBody<UpdateArticle>>,
) -> Result<Json<ArticleBody>> {
    let mut tx = ctx.db.begin().await?;

    let new_slug = req.article.title.as_deref().map(slugify);

    metrics::observe_db_query();
    let meta_row = sqlx::query(
        "select article_id, user_id from article where slug = $1 for update",
        slug
    )
    .fetch_optional(&mut *tx)
    .await?
    .ok_or(Error::NotFound)?;

    if meta_row.get::<uuid::Uuid, _>("user_id") != auth_user.user_id {
        return Err(Error::Forbidden);
    }

    let article_id = meta_row.get::<uuid::Uuid, _>("article_id");

    metrics::observe_db_query();
    let row = sqlx::query(
        r#"
            with updated_article as (
                update article
                set
                    slug = coalesce($1, slug),
                    title = coalesce($2, title),
                    description = coalesce($3, description),
                    body = coalesce($4, body)
                where article_id = $5
                returning
                    slug,
                    title,
                    description,
                    body,
                    tag_list,
                    created_at,
                    updated_at
            )
            select
                updated_article.*,
                exists(select 1 from article_favorite where user_id = $6) as favorited,
                coalesce(
                    (select count(*) from article_favorite fav where fav.article_id = $5),
                    0
                ) as favorites_count,
                author.username as author_username,
                author.bio as author_bio,
                author.pfp_id as author_image,
                false as following_author
            from updated_article
            inner join "user" author on author.user_id = $6
        "#,
        new_slug,
        req.article.title,
        req.article.description,
        req.article.body,
        article_id,
        auth_user.user_id
    )
    .fetch_one(&mut *tx)
    .await
    .on_constraint("article_slug_key", |_| {
        Error::unprocessable_entity([(
            "slug",
            format!("duplicate article slug: {}", new_slug.unwrap()),
        )])
    })?;

    let article = ArticleFromQuery {
        slug: row.get::<String, _>("slug"),
        title: row.get::<String, _>("title"),
        description: row.get::<String, _>("description"),
        body: row.get::<String, _>("body"),
        tag_list: row.get::<Vec<String>, _>("tag_list"),
        created_at: row.get::<Timestamptz, _>("created_at"),
        updated_at: row.get::<Timestamptz, _>("updated_at"),
        favorited: row.get::<bool, _>("favorited"),
        favorites_count: row.get::<i64, _>("favorites_count"),
        author_username: row.get::<String, _>("author_username"),
        author_bio: row.get::<Option<String>, _>("author_bio").unwrap_or_default(),
        author_image: row.get::<Option<uuid::Uuid>, _>("author_image"),
        following_author: row.get::<bool, _>("following_author"),
    };

    tx.commit().await?;

    Ok(Json(ArticleBody { article: article.into_article() }))
}

async fn delete_article(
    auth_user: AuthUser,
    State(ctx): State<ApiContext>,
    Path(slug): Path<String>,
) -> Result<()> {
    metrics::observe_db_query();
    let row = sqlx::query(
        r#"
            with deleted_article as (
                delete from article 
                where slug = $1 and user_id = $2
                returning 1
            )
            select
                exists(select 1 from article where slug = $1) as existed,
                exists(select 1 from deleted_article) as deleted
        "#,
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

async fn get_article(
    maybe_auth_user: MaybeAuthUser,
    State(ctx): State<ApiContext>,
    Path(slug): Path<String>,
) -> Result<Json<ArticleBody>> {
    metrics::observe_db_query();
    let row = sqlx::query(
        r#"
            select
                slug,
                title,
                description,
                body,
                tag_list,
                article.created_at,
                article.updated_at,
                exists(select 1 from article_favorite where user_id = $1) as favorited,
                coalesce(
                    (select count(*) from article_favorite fav where fav.article_id = article.article_id),
                    0
                ) as favorites_count,
                author.username as author_username,
                author.bio as author_bio,
                author.pfp_id as author_image,
                exists(select 1 from follow where followed_user_id = author.user_id and following_user_id = $1) as following_author
            from article
            inner join "user" author using (user_id)
            where slug = $2
        "#,
        maybe_auth_user.user_id(),
        slug
    )
        .fetch_optional(&ctx.db)
        .await?
        .ok_or(Error::NotFound)?;

    let article = ArticleFromQuery {
        slug: row.get::<String, _>("slug"),
        title: row.get::<String, _>("title"),
        description: row.get::<String, _>("description"),
        body: row.get::<String, _>("body"),
        tag_list: row.get::<Vec<String>, _>("tag_list"),
        created_at: row.get::<Timestamptz, _>("created_at"),
        updated_at: row.get::<Timestamptz, _>("updated_at"),
        favorited: row.get::<bool, _>("favorited"),
        favorites_count: row.get::<i64, _>("favorites_count"),
        author_username: row.get::<String, _>("author_username"),
        author_bio: row.get::<Option<String>, _>("author_bio").unwrap_or_default(),
        author_image: row.get::<Option<uuid::Uuid>, _>("author_image"),
        following_author: row.get::<bool, _>("following_author"),
    };

    Ok(Json(ArticleBody { article: article.into_article() }))
}

async fn favorite_article(
    auth_user: AuthUser,
    State(ctx): State<ApiContext>,
    Path(slug): Path<String>,
) -> Result<()> {
    metrics::observe_db_query();
    let _: Option<uuid::Uuid> = sqlx::query_scalar(
        r#"
            with selected_article as (
                select article_id from article where slug = $1
            ),
            inserted_favorite as (
                insert into article_favorite(article_id, user_id)
                select article_id, $2
                from selected_article
                on conflict do nothing
            )
            select article_id from selected_article
        "#,
        slug,
        auth_user.user_id
    )
    .fetch_optional(&ctx.db)
    .await?
    .ok_or(Error::NotFound)?;

    Ok(())
}

async fn unfavorite_article(
    auth_user: AuthUser,
    State(ctx): State<ApiContext>,
    Path(slug): Path<String>,
) -> Result<()> {
    metrics::observe_db_query();
    let _: Option<uuid::Uuid> = sqlx::query_scalar(
        r#"
            with selected_article as (
                select article_id from article where slug = $1
            ),
            deleted_favorite as (
                delete from article_favorite
                where article_id = (select article_id from selected_article)
                and user_id = $2
            )
            select article_id from selected_article
        "#,
        slug,
        auth_user.user_id
    )
    .fetch_optional(&ctx.db)
    .await?
    .ok_or(Error::NotFound)?;

    Ok(())
}

async fn get_tags(State(ctx): State<ApiContext>) -> Result<Json<TagsBody>> {
    metrics::observe_db_query();
    let rows = sqlx::query(
        r#"
            select distinct tag
            from article, unnest (article.tag_list) tags(tag)
            order by tag
        "#
    )
    .fetch_all(&ctx.db)
    .await?;

    let tags: Vec<String> = rows.into_iter().map(|r| r.get::<String, _>("tag")).collect();

    Ok(Json(TagsBody { tags }))
}

async fn _article_by_id(
    e: impl Executor<'_, Database = Postgres>,
    user_id: Uuid,
    article_id: Uuid,
) -> Result<Article> {
    metrics::observe_db_query();
    let row = sqlx::query(
        r#"
            select
                slug,
                title,
                description,
                body,
                tag_list,
                article.created_at,
                article.updated_at,
                exists(select 1 from article_favorite where user_id = $1) as favorited,
                coalesce(
                    (select count(*) from article_favorite fav where fav.article_id = article.article_id),
                    0
                ) as favorites_count,
                author.username as author_username,
                author.bio as author_bio,
                author.pfp_id as author_image,
                exists(select 1 from follow where followed_user_id = author.user_id and following_user_id = $1) as following_author
            from article
            inner join "user" author using (user_id)
            where article_id = $2
        "#,
        user_id,
        article_id
    )
        .fetch_optional(e)
        .await?
        .ok_or(Error::NotFound)?;

    let article = ArticleFromQuery {
        slug: row.get::<String, _>("slug"),
        title: row.get::<String, _>("title"),
        description: row.get::<String, _>("description"),
        body: row.get::<String, _>("body"),
        tag_list: row.get::<Vec<String>, _>("tag_list"),
        created_at: row.get::<Timestamptz, _>("created_at"),
        updated_at: row.get::<Timestamptz, _>("updated_at"),
        favorited: row.get::<bool, _>("favorited"),
        favorites_count: row.get::<i64, _>("favorites_count"),
        author_username: row.get::<String, _>("author_username"),
        author_bio: row.get::<Option<String>, _>("author_bio").unwrap_or_default(),
        author_image: row.get::<Option<uuid::Uuid>, _>("author_image"),
        following_author: row.get::<bool, _>("following_author"),
    };

    Ok(article.into_article())
}

fn slugify(string: &str) -> String {
    const QUOTE_CHARS: &[char] = &['\'', '"'];

    string
        // Split on anything that isn't a word character or quotation mark.
        // This has the effect of keeping contractions and possessives together.
        .split(|c: char| !(QUOTE_CHARS.contains(&c) || c.is_alphanumeric()))
        // If multiple non-word characters follow each other then we'll get empty substrings
        // so we'll filter those out.
        .filter(|s| !s.is_empty())
        .map(|s| {
            // Remove quotes from the substring.
            //
            // This allocation is probably avoidable with some more iterator hackery but
            // at that point we'd be micro-optimizing. This function isn't called all that often.
            let mut s = s.replace(QUOTE_CHARS, "");
            // Make the substring lowercase (in-place operation)
            s.make_ascii_lowercase();
            s
        })
        .join("-")
}

#[test]
fn test_slugify() {
    assert_eq!(
        slugify("Segfaults and You: When Raw Pointers Go Wrong"),
        "segfaults-and-you-when-raw-pointers-go-wrong"
    );

    assert_eq!(
        slugify("Why are DB Admins Always Shouting?"),
        "why-are-db-admins-always-shouting"
    );

    assert_eq!(
        slugify("Converting to Rust from C: It's as Easy as 1, 2, 3!"),
        "converting-to-rust-from-c-its-as-easy-as-1-2-3"
    )
}
