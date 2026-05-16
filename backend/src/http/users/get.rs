use crate::http::{ApiContext, Result};
use crate::metrics;
use axum::Json;
use axum::extract::{Path, Query, State};

use crate::http::error::Error;
use crate::http::extractor::AuthUser;
use crate::http::materials::models as materials_models;
use chrono::{DateTime, Utc};
use sqlx::Row;

use super::models::*;

#[utoipa::path(
    get,
    path = "/api/users/me",
    tag = "users",
    responses(
        (status = 200, description = "Current user profile", body = WebUser),
        (status = 401, description = "Authentication required", body = crate::http::error::ApiError),
        (status = 500, description = "Internal server error", body = crate::http::error::ApiError)
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_current_user(
    auth_user: AuthUser,
    State(ctx): State<ApiContext>,
) -> Result<Json<WebUser>> {
    metrics::observe_db_query();
    let row = sqlx::query(
        r#"
            select user_id, email, name, bio, roles, is_email_verified
            from web_user where user_id = $1
        "#,
    )
    .bind(auth_user.user_id)
    .fetch_one(&ctx.db)
    .await?;

    let roles: Vec<String> = row
        .get::<Option<Vec<String>>, _>("roles")
        .unwrap_or_else(|| vec!["user".to_string()]);

    Ok(Json(WebUser {
        id: row.get::<uuid::Uuid, _>("user_id"),
        name: row.get::<String, _>("name"),
        email: row.get::<String, _>("email"),
        bio: row.get::<Option<String>, _>("bio").unwrap_or_default(),
        is_email_verified: row.get::<bool, _>("is_email_verified"),
        can_submit_materials: row.get::<bool, _>("is_email_verified"),
        roles,
    }))
}

#[utoipa::path(
    get,
    path = "/api/users",
    tag = "users",
    params(ListUsersQuery),
    responses(
        (status = 200, description = "Public users list", body = PaginatedUsersResponse),
        (status = 500, description = "Internal server error", body = crate::http::error::ApiError)
    )
)]
pub async fn get_users(
    Query(query): Query<ListUsersQuery>,
    State(ctx): State<ApiContext>,
) -> Result<Json<PaginatedUsersResponse>> {
    let page = query.page.unwrap_or(1).max(1);
    let limit = query.limit.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * limit;

    let search_pattern = query.search.map(|s| format!("%{}%", s));

    let total: i64 = sqlx::query_scalar(
        r#"
            select count(distinct u.user_id)::int8
            from web_user u
            inner join material m on m.user_id = u.user_id
            where m.published_at is not null
            and ($1::text is null or u.name ilike $1)
        "#,
    )
    .bind(&search_pattern)
    .fetch_one(&ctx.db)
    .await?;

    let rows = sqlx::query(
        r#"
            select 
                u.user_id,
                u.name,
                u.bio,
                u.is_email_verified,
                count(m.material_id)::int8 as materials_count
            from web_user u
            inner join material m on m.user_id = u.user_id
            where m.published_at is not null
            and ($1::text is null or u.name ilike $1)
            group by u.user_id, u.name, u.bio, u.is_email_verified
            order by u.name
            limit $2 offset $3
        "#,
    )
    .bind(&search_pattern)
    .bind(limit)
    .bind(offset)
    .fetch_all(&ctx.db)
    .await?;

    let items: Vec<UserPublicProfile> = rows
        .into_iter()
        .map(|u| UserPublicProfile {
            id: u.get::<uuid::Uuid, _>("user_id"),
            name: u.get::<String, _>("name"),
            bio: u.get::<Option<String>, _>("bio").unwrap_or_default(),
            is_email_verified: u.get::<bool, _>("is_email_verified"),
            materials_count: u.get::<i64, _>("materials_count"),
        })
        .collect();

    Ok(Json(PaginatedUsersResponse {
        items,
        page,
        limit,
        total,
    }))
}

#[utoipa::path(
    get,
    path = "/api/users/{userId}",
    tag = "users",
    params(
        ("userId" = uuid::Uuid, Path, description = "User id"),
        ListUsersQuery
    ),
    responses(
        (status = 200, description = "Public user profile with materials", body = UserWithMaterialsResponse),
        (status = 404, description = "User not found", body = crate::http::error::ApiError),
        (status = 500, description = "Internal server error", body = crate::http::error::ApiError)
    )
)]
pub async fn get_user_by_id(
    Path(user_id): Path<uuid::Uuid>,
    Query(query): Query<ListUsersQuery>,
    State(ctx): State<ApiContext>,
) -> Result<Json<UserWithMaterialsResponse>> {
    let page = query.page.unwrap_or(1).max(1);
    let limit = query.limit.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * limit;

    let user_row = sqlx::query(
        r#"
            select user_id, name, bio, is_email_verified
            from web_user where user_id = $1
        "#,
    )
    .bind(user_id)
    .fetch_optional(&ctx.db)
    .await?
    .ok_or(Error::NotFound)?;

    let user_name = user_row.get::<String, _>("name");
    let user_bio: String = user_row.get::<Option<String>, _>("bio").unwrap_or_default();
    let user_is_verified = user_row.get::<bool, _>("is_email_verified");

    let materials_count: i64 = sqlx::query_scalar(
        "select count(*)::int8 from material where user_id = $1 and published_at is not null",
    )
    .bind(user_id)
    .fetch_one(&ctx.db)
    .await?;

    let limit_plus_one = limit + 1;
    let material_rows = sqlx::query(
        r#"
            select
                material_id,
                user_id,
                title,
                description,
                courses,
                subjects,
                type,
                difficulty,
                published_at
            from material
            where user_id = $1 and published_at is not null
            order by published_at desc
            limit $2 offset $3
        "#,
    )
    .bind(user_id)
    .bind(limit_plus_one)
    .bind(offset)
    .fetch_all(&ctx.db)
    .await?;

    let total = materials_count;
    let _has_more = material_rows.len() > limit as usize;
    let items: Vec<materials_models::Material> = material_rows
        .into_iter()
        .take(limit as usize)
        .map(|m| {
            let pub_date: Option<String> = m
                .get::<Option<DateTime<Utc>>, _>("published_at")
                .map(|dt| dt.format("%Y-%m-%dT%H:%M:%S%.fZ").to_string());
            materials_models::Material {
                id: m.get::<uuid::Uuid, _>("material_id"),
                author_id: m.get::<uuid::Uuid, _>("user_id"),
                author_name: Some(user_name.clone()),
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

    let user_profile = UserPublicProfile {
        id: user_row.get::<uuid::Uuid, _>("user_id"),
        name: user_name,
        bio: user_bio,
        is_email_verified: user_is_verified,
        materials_count,
    };

    Ok(Json(UserWithMaterialsResponse {
        user: user_profile,
        materials: materials_models::PaginatedMaterialsResponse {
            items,
            page,
            limit,
            total,
        },
    }))
}
