use crate::http::{ApiContext, Result};
use axum::extract::State;

use crate::http::error::Error;
use crate::http::extractor::AuthUser;
use redis::AsyncCommands;

#[utoipa::path(
    post,
    path = "/api/auth/logout",
    tag = "auth",
    responses(
        (status = 200, description = "Logged out successfully"),
        (status = 401, description = "Authentication required", body = crate::http::error::ApiError),
        (status = 500, description = "Internal server error", body = crate::http::error::ApiError)
    ),
    security(("bearer_auth" = []))
)]
pub async fn logout(auth_user: AuthUser, State(ctx): State<ApiContext>) -> Result<()> {
    let mut redis = ctx.redis.clone();
    let key = format!("session:{}", auth_user.session_id);
    let _: () = redis
        .del(key)
        .await
        .map_err(|_| Error::Anyhow(anyhow::anyhow!("redis error")))?;

    Ok(())
}
