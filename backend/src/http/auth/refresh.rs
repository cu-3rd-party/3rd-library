use crate::http::{ApiContext, Result};
use anyhow::anyhow;
use axum::Json;
use axum::extract::State;

use super::models::*;
use crate::constants::DEFAULT_SESSION_LENGTH;
use crate::http::error::Error;
use crate::http::extractor::AuthUser;
use redis::AsyncCommands;
use uuid::Uuid;

#[utoipa::path(
    post,
    path = "/api/auth/refresh",
    tag = "auth",
    request_body = RefreshTokenRequest,
    responses(
        (status = 200, description = "Tokens refreshed", body = TokenPair),
        (status = 401, description = "Invalid refresh token", body = crate::http::error::ApiError),
        (status = 500, description = "Internal server error", body = crate::http::error::ApiError)
    )
)]
pub async fn refresh_token(
    State(ctx): State<ApiContext>,
    Json(req): Json<RefreshTokenRequest>,
) -> Result<Json<TokenPair>> {
    let session_id = Uuid::parse_str(&req.refresh_token)
        .map_err(|_| Error::unauthorized("invalid_token", "Invalid refresh token"))?;

    let mut redis = ctx.redis.clone();
    let key = format!("session:{}", session_id);
    let cached_user_str: Option<String> = redis
        .get(&key)
        .await
        .map_err(|_| Error::unauthorized("invalid_token", "Invalid refresh token"))?;

    let cached_user_str = cached_user_str
        .ok_or_else(|| Error::unauthorized("invalid_token", "Invalid refresh token"))?;

    let mut auth_user: AuthUser =
        serde_json::from_str(&cached_user_str).map_err(|err| anyhow!(err))?;

    let new_session_id = Uuid::new_v4();
    auth_user.session_id = new_session_id;

    let _: () = redis.unlink(&key).await.map_err(|err| anyhow!(err))?;

    let tokens = TokenPair {
        access_token: auth_user.to_jwt(&ctx).await?,
        refresh_token: new_session_id.to_string(),
        expires_in: DEFAULT_SESSION_LENGTH.num_seconds().max(1) as u64,
    };

    Ok(Json(tokens))
}
