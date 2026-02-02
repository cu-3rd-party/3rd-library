use axum::Json;
use axum::extract::{Query, State};
use futures::TryStreamExt;

use crate::http;
use crate::http::ApiContext;
use crate::http::articles::{Article, ArticleFromQuery};
use crate::http::extractor::{AuthUser, MaybeAuthUser};
#[allow(unused_imports)]
use crate::http::types::Timestamptz;
use crate::metrics;

#[derive(serde::Deserialize, Default)]
#[serde(default)]
pub struct ListArticlesQuery {
    // TODO: Theoretically we could allow filtering by multiple tags, e.g. `/api/articles?tag=Rust&tag=SQL`
    tag: Option<String>,
    author: Option<String>,
    favorited: Option<String>,

    // TODO:
    //      `limit` and `offset` are not the optimal way to paginate SQL queries, because the query
    //      planner essentially has to fetch the whole dataset first and then cull it afterwards.
    //
    //      It's a much better idea to paginate using the value of an indexed column.
    //      For articles, that could be `created_at`, keeping `limit` and then repeatedly querying
    //      for `created_at < oldest_created_at_of_previous_query`.
    //
    //      Since the spec doesn't return a JSON array at the top level, you could have a `next`
    //      field after `articles` that is the URL that the frontend should fetch to get the next page in
    //      the ordering, so the frontend doesn't even need to care what column you're using to paginate.
    limit: Option<i64>,
    offset: Option<i64>,
}

#[derive(serde::Deserialize, Default)]
#[serde(default)]
pub struct FeedArticlesQuery {
    limit: Option<i64>,
    offset: Option<i64>,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MultipleArticlesBody {
    articles: Vec<Article>,

    articles_count: usize,
}

pub(in crate::http) async fn list_articles(
    // authentication is optional
    maybe_auth_user: MaybeAuthUser,
    State(ctx): State<ApiContext>,
    query: Query<ListArticlesQuery>,
) -> http::Result<Json<MultipleArticlesBody>> {
    metrics::observe_db_query();
    let articles: Vec<_> = sqlx::query_as!(
        ArticleFromQuery,
        r#"
            select
                slug,
                title,
                description,
                body,
                tag_list,
                article.created_at "created_at: Timestamptz",
                article.updated_at "updated_at: Timestamptz",
                exists(select 1 from article_favorite where user_id = $1) "favorited!",
                coalesce(
                    -- `count(*)` returns `NULL` if the query returned zero columns
                    -- not exactly a fan of that design choice but whatever
                    (select count(*) from article_favorite fav where fav.article_id = article.article_id),
                    0
                ) "favorites_count!",
                author.username author_username,
                author.bio author_bio,
                author.image author_image,
                exists(select 1 from follow where followed_user_id = author.user_id and following_user_id = $1) "following_author!"
            from article
            inner join "user" author using (user_id)
            -- the current way to do conditional filtering in SQLx
            where (
                -- check if `query.tag` is null or contains the given tag
                -- PostgresSQL doesn't have an "array contains element" operator
                -- so instead we check if the tag_list contains an array of just the given tag
                $2::text is null or tag_list @> array[$2]
            )
              and
            (
                $3::text is null or author.username = $3
            )
              and
            (
                $4::text is null or exists(
                    select 1
                    from "user"
                    inner join article_favorite af using (user_id)
                    where username = $4
                )
            )
            order by article.created_at desc
            limit $5
            offset $6
        "#,
        maybe_auth_user.user_id(),
        query.tag,
        query.author,
        query.favorited,
        query.limit.unwrap_or(20),
        query.offset.unwrap_or(0)
    )
        .fetch(&ctx.db)
        .map_ok(ArticleFromQuery::into_article)
        .try_collect()
        .await?;

    Ok(Json(MultipleArticlesBody {
        articles_count: articles.len(),
        articles,
    }))
}

pub(in crate::http) async fn feed_articles(
    auth_user: AuthUser,
    State(ctx): State<ApiContext>,
    query: Query<FeedArticlesQuery>,
) -> http::Result<Json<MultipleArticlesBody>> {
    metrics::observe_db_query();
    let articles: Vec<_> = sqlx::query_as!(
        ArticleFromQuery,
        r#"
            select
                slug,
                title,
                description,
                body,
                tag_list,
                article.created_at "created_at: Timestamptz",
                article.updated_at "updated_at: Timestamptz",
                exists(select 1 from article_favorite where user_id = $1) "favorited!",
                coalesce(
                    (select count(*) from article_favorite fav where fav.article_id = article.article_id),
                    0
                ) "favorites_count!",
                author.username author_username,
                author.bio author_bio,
                author.image author_image,
                -- we wouldn't be returning this otherwise
                true "following_author!"
            from follow
            inner join article on followed_user_id = article.user_id
            inner join "user" author using (user_id)
            where following_user_id = $1
            limit $2
            offset $3
        "#,
        auth_user.user_id,
        query.limit.unwrap_or(20),
        query.offset.unwrap_or(0)
    )
        .fetch(&ctx.db)
        .map_ok(ArticleFromQuery::into_article)
        .try_collect()
        .await?;

    Ok(Json(MultipleArticlesBody {
        articles_count: articles.len(),
        articles,
    }))
}
