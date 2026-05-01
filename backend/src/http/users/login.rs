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
    let row = sqlx::query(
        r#"
            select user_id, email, username, bio, pfp_id, password_hash
            from "user" where email = $1
        "#,
        req.user.email,
    )
    .fetch_optional(&ctx.db)
    .await?
    .ok_or(Error::unprocessable_entity([("email", "does not exist")]))?;

    verify_password(req.user.password, row.get::<String, _>("password_hash")).await?;

    Ok(Json(UserBody {
        user: User {
            email: row.get::<String, _>("email"),
            token: AuthUser {
                user_id: row.get::<uuid::Uuid, _>("user_id"),
                session_id: Uuid::new_v4(),
            }
            .to_jwt(&ctx)
            .await?,
            username: row.get::<String, _>("username"),
            bio: row.get::<Option<String>, _>("bio"),
            image: row.get::<Option<uuid::Uuid>, _>("pfp_id"),
        },
    }))
}
