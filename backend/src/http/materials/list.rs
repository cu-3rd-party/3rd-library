use crate::http::{ApiContext, Result};
use axum::Json;
use axum::extract::{Query, State};

use crate::http::extractor::{AuthUser, MaybeAuthUser};
use chrono::{DateTime, Utc};
use sqlx::Row;

use super::models::*;

fn to_rfc3339(dt: DateTime<Utc>) -> String {
    dt.format("%Y-%m-%dT%H:%M:%S%.fZ").to_string()
}

pub async fn list_materials(
    Query(query): Query<ListMaterialsQuery>,
    _maybe_auth_user: MaybeAuthUser,
    State(ctx): State<ApiContext>,
) -> Result<Json<PaginatedMaterialsResponse>> {
    let page = query.page.unwrap_or(1).max(1);
    let limit = query.limit.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * limit;

    let search_pattern = query.search.map(|s| format!("%{}%", s));

    let total: i64 = sqlx::query_scalar(
        r#"
            select count(*)::int8 from material
            where published_at is not null
            and ($1::text is null or title ilike $1)
        "#,
    )
    .bind(&search_pattern)
    .fetch_one(&ctx.db)
    .await?;

    let limit_plus_one = limit + 1;
    let rows = sqlx::query(
        r#"
            select
                m.material_id,
                m.user_id,
                u.name as author_name,
                m.title,
                m.description,
                m.courses,
                m.subjects,
                m.type,
                m.difficulty,
                m.published_at
            from material m
            inner join web_user u on m.user_id = u.user_id
            where m.published_at is not null
            and ($1::text is null or m.title ilike $1)
            order by m.published_at desc
            limit $2 offset $3
        "#,
    )
    .bind(&search_pattern)
    .bind(limit_plus_one)
    .bind(offset)
    .fetch_all(&ctx.db)
    .await?;

    let items: Vec<Material> = rows
        .into_iter()
        .take(limit as usize)
        .map(|m| {
            let pub_date: Option<String> = m
                .get::<Option<DateTime<Utc>>, _>("published_at")
                .map(to_rfc3339);
            Material {
                id: m.get::<uuid::Uuid, _>("material_id"),
                author_id: m.get::<uuid::Uuid, _>("user_id"),
                author_name: m.get::<Option<String>, _>("author_name"),
                title: m.get::<String, _>("title"),
                description: m
                    .get::<Option<String>, _>("description")
                    .unwrap_or_default(),
                courses: m
                    .get::<Option<Vec<String>>, _>("courses")
                    .unwrap_or_default(),
                subjects: m
                    .get::<Option<Vec<String>>, _>("subjects")
                    .unwrap_or_default(),
                r#type: m.get::<String, _>("type"),
                difficulty: m
                    .get::<Option<String>, _>("difficulty")
                    .unwrap_or_else(|| "none".to_string()),
                pub_date,
            }
        })
        .collect();

    Ok(Json(PaginatedMaterialsResponse {
        items,
        page,
        limit,
        total,
    }))
}

pub async fn list_submissions(
    auth_user: AuthUser,
    Query(query): Query<ListSubmissionsQuery>,
    State(ctx): State<ApiContext>,
) -> Result<Json<PaginatedSubmissionsResponse>> {
    let page = query.page.unwrap_or(1).max(1);
    let limit = query.limit.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * limit;

    let total: i64 = if query.status.as_deref() == Some("all") || query.status.is_none() {
        sqlx::query_scalar("select count(*)::int8 from submission s where s.user_id = $1")
            .bind(auth_user.user_id)
            .fetch_one(&ctx.db)
            .await?
    } else {
        sqlx::query_scalar(
            "select count(*)::int8 from submission s where s.user_id = $1 and s.status = $2",
        )
        .bind(auth_user.user_id)
        .bind(query.status.as_ref().unwrap())
        .fetch_one(&ctx.db)
        .await?
    };

    let limit_plus_one = limit + 1;
    let rows = sqlx::query(
        r#"
            select
                s.submission_id,
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
                s.reviewed_at
            from submission s
            where s.user_id = $1
            order by s.created_at desc
            limit $2 offset $3
        "#,
    )
    .bind(auth_user.user_id)
    .bind(limit_plus_one)
    .bind(offset)
    .fetch_all(&ctx.db)
    .await?;

    let items: Vec<Submission> = rows
        .into_iter()
        .take(limit as usize)
        .map(|s| s.into())
        .collect();

    Ok(Json(PaginatedSubmissionsResponse {
        items,
        page,
        limit,
        total,
    }))
}
