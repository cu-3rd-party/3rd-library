use crate::http::{ApiContext, Result};
use crate::metrics;
use axum::Json;
use axum::extract::State;

use crate::http::error::{Error, ResultExt};
use crate::http::extractor::AuthUser;

use super::get::get_current_user;
use super::helpers::*;
use super::models::*;

pub async fn update_user(
    auth_user: AuthUser,
    State(ctx): State<ApiContext>,
    Json(req): Json<UserBody<UpdateUser>>,
) -> Result<Json<UserBody<User>>> {
    if req.user == UpdateUser::default() {
        return get_current_user(auth_user, State(ctx)).await;
    }

    let password_hash = if let Some(password) = req.user.password {
        Some(hash_password(password).await?)
    } else {
        None
    };

    let requested_pfp_id = req.user.image;
    if let Some(pfp_id) = requested_pfp_id {
        let owns_pfp = sqlx::query_scalar!(
            r#"
                select exists(
                    select 1
                    from profile_picture
                    where pfp_id = $1 and user_id = $2
                ) as "exists!"
            "#,
            pfp_id,
            auth_user.user_id
        )
        .fetch_one(&ctx.db)
        .await?;

        if !owns_pfp {
            return Err(Error::unprocessable_entity([(
                "image",
                "invalid profile picture",
            )]));
        }
    }

    metrics::observe_db_query();
    let user = sqlx::query!(
        r#"
            update "user"
            set email = coalesce($1, "user".email),
                username = coalesce($2, "user".username),
                password_hash = coalesce($3, "user".password_hash),
                bio = coalesce($4, "user".bio),
                pfp_id = coalesce($5, "user".pfp_id)
            where user_id = $6
            returning email, username, bio, pfp_id
        "#,
        req.user.email,
        req.user.username,
        password_hash,
        req.user.bio,
        requested_pfp_id,
        auth_user.user_id
    )
    .fetch_one(&ctx.db)
    .await
    .on_constraint("user_username_key", |_| {
        Error::unprocessable_entity([("username", "username taken")])
    })
    .on_constraint("user_email_key", |_| {
        Error::unprocessable_entity([("email", "email taken")])
    })?;

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
