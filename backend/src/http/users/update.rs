use crate::http::{ApiContext, Result};
use axum::Json;
use axum::extract::State;
use serde::Deserialize;

use crate::http::extractor::AuthUser;
use sqlx::Row;

use super::models::*;

#[derive(Deserialize)]
pub struct UpdateUserRequest {
    pub name: Option<String>,
    pub bio: Option<String>,
}

pub async fn update_user_profile(
    auth_user: AuthUser,
    State(ctx): State<ApiContext>,
    Json(req): Json<UpdateUserRequest>,
) -> Result<Json<WebUser>> {
    sqlx::query(
        "update web_user set name = coalesce($1, name), bio = coalesce($2, bio) where user_id = $3",
    )
    .bind(&req.name)
    .bind(&req.bio)
    .bind(auth_user.user_id)
    .execute(&ctx.db)
    .await?;

    let row = sqlx::query(
        "select user_id, email, name, bio, roles, is_email_verified from web_user where user_id = $1"
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
