use crate::http::{ApiContext, Result};
use crate::metrics;
use axum::Json;
use axum::extract::State;

use crate::http::extractor::AuthUser;

use super::models::*;

pub async fn get_current_user(
    auth_user: AuthUser,
    State(ctx): State<ApiContext>,
) -> Result<Json<UserBody<User>>> {
    metrics::observe_db_query();
    let user = sqlx::query!(
        r#"select email, username, bio, pfp_id from "user" where user_id = $1"#,
        auth_user.user_id
    )
    .fetch_one(&ctx.db)
    .await?;

    Ok(Json(UserBody {
        user: User {
            email: user.email,
            token: auth_user.to_jwt(&ctx).await?,
            username: user.username,
            bio: user.bio,
            image: user.pfp_id,
        },
    }))
}
