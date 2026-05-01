use crate::http::{ApiContext, Result};
use crate::metrics;
use axum::Json;
use axum::extract::State;

use crate::http::error::{Error, ResultExt};
use crate::http::extractor::AuthUser;
use uuid::Uuid;

use super::helpers::*;
use super::models::*;

pub async fn create_user(
    State(ctx): State<ApiContext>,
    Json(req): Json<UserBody<NewUser>>,
) -> Result<Json<UserBody<User>>> {
    let password_hash = hash_password(req.user.password).await?;

    metrics::observe_db_query();
    let user_id: uuid::Uuid = sqlx::query_scalar(
        r#"insert into "user" (username, email, password_hash) values ($1, $2, $3) returning user_id"#,
        req.user.username,
        req.user.email,
        password_hash
    )
    .fetch_one(&ctx.db)
    .await
    .on_constraint("user_username_key", |_| {
        Error::unprocessable_entity([("username", "username taken")])
    })
    .on_constraint("user_email_key", |_| {
        Error::unprocessable_entity([("email", "email taken")])
    })?;

    metrics::record_user_created();

    Ok(Json(UserBody {
        user: User {
            email: req.user.email,
            token: AuthUser {
                user_id,
                session_id: Uuid::new_v4(),
            }
            .to_jwt(&ctx)
            .await?,
            username: req.user.username,
            bio: "".to_string(),
            image: None,
        },
    }))
}
