use crate::http::{ApiContext, Result};
use crate::metrics;
use axum::Json;
use axum::extract::State;

use crate::http::error::Error;
use crate::http::extractor::AuthUser;
use uuid::Uuid;

use super::helpers::*;
use super::models::*;

pub async fn login_user(
    State(ctx): State<ApiContext>,
    Json(req): Json<UserBody<LoginUser>>,
) -> Result<Json<UserBody<User>>> {
    metrics::observe_db_query();
    let user = sqlx::query!(
        r#"
            select user_id, email, username, bio, pfp_id, password_hash
            from "user" where email = $1
        "#,
        req.user.email,
    )
    .fetch_optional(&ctx.db)
    .await?
    .ok_or(Error::unprocessable_entity([("email", "does not exist")]))?;

    verify_password(req.user.password, user.password_hash).await?;

    Ok(Json(UserBody {
        user: User {
            email: user.email,
            token: AuthUser {
                user_id: user.user_id,
                session_id: Uuid::new_v4(),
            }
            .to_jwt(&ctx)
            .await?,
            username: user.username,
            bio: user.bio,
            image: user.pfp_id,
        },
    }))
}
