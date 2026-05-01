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
    maybe_auth_user: MaybeAuthUser,
    State(ctx): State<ApiContext>,
    query: Query<ListArticlesQuery>,
) -> http::Result<Json<MultipleArticlesBody>> {
    metrics::observe_db_query();
    let rows = sqlx::query(
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
            where (
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
        .map_ok(|row| {
            ArticleFromQuery {
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
            }.into_article()
        })
        .try_collect()
        .await?;

    Ok(Json(MultipleArticlesBody {
        articles_count: rows.len(),
        articles: rows,
    }))
}

pub(in crate::http) async fn feed_articles(
    auth_user: AuthUser,
    State(ctx): State<ApiContext>,
    query: Query<FeedArticlesQuery>,
) -> http::Result<Json<MultipleArticlesBody>> {
    metrics::observe_db_query();
    let rows = sqlx::query(
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
                true as following_author
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
        .map_ok(|row| {
            ArticleFromQuery {
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
            }.into_article()
        })
        .try_collect()
        .await?;

    Ok(Json(MultipleArticlesBody {
        articles_count: rows.len(),
        articles: rows,
    }))
}
