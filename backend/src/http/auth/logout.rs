use crate::http::{ApiContext, Result};
use axum::extract::State;

use crate::http::error::Error;
use crate::http::extractor::AuthUser;
use redis::AsyncCommands;

pub async fn logout(auth_user: AuthUser, State(ctx): State<ApiContext>) -> Result<()> {
    let mut redis = ctx.redis.clone();
    let key = format!("session:{}", auth_user.session_id);
    let _: () = redis
        .del(key)
        .await
        .map_err(|_| Error::Anyhow(anyhow::anyhow!("redis error")))?;

    Ok(())
}
