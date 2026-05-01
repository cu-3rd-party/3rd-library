use crate::http::{ApiContext, Result};
use axum::Json;
use axum::extract::State;

use crate::http::error::Error;
use crate::http::extractor::AuthUser;
use redis::AsyncCommands;
use uuid::Uuid;

use super::models::*;

pub async fn refresh_token(
    State(ctx): State<ApiContext>,
    Json(req): Json<RefreshTokenRequest>,
) -> Result<Json<TokenPair>> {
    let session_id = Uuid::parse_str(&req.refresh_token)
        .map_err(|_| Error::unauthorized("invalid_token", "Invalid refresh token"))?;

    let mut redis = ctx.redis.clone();
    let key = format!("session:{}", session_id);
    let user_id_str: Option<String> = redis
        .get(&key)
        .await
        .map_err(|_| Error::unauthorized("invalid_token", "Invalid refresh token"))?;

    let user_id_str =
        user_id_str.ok_or_else(|| Error::unauthorized("invalid_token", "Invalid refresh token"))?;

    let user_id = Uuid::parse_str(&user_id_str)
        .map_err(|_| Error::unauthorized("invalid_token", "Invalid refresh token"))?;

    let new_session_id = Uuid::new_v4();

    let ttl_seconds = 1209600u64;
    let _: () = redis
        .set_ex(&key, new_session_id.to_string(), ttl_seconds)
        .await
        .map_err(|_| Error::unauthorized("invalid_token", "Invalid refresh token"))?;

    let auth_user = AuthUser {
        user_id,
        session_id: new_session_id,
    };

    let tokens = TokenPair {
        access_token: auth_user.to_jwt(&ctx).await?,
        refresh_token: new_session_id.to_string(),
        expires_in: 1209600,
    };

    Ok(Json(tokens))
}
