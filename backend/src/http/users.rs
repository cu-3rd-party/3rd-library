use crate::http::{ApiContext, Result};
use crate::metrics;
use anyhow::Context;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash};
use axum::extract::State;
use axum::routing::{get, post};
use axum::{Json, Router};

use crate::http::error::{Error, ResultExt};
use crate::http::extractor::AuthUser;
use uuid::Uuid;

pub fn router() -> Router<ApiContext> {
    Router::new()
        .route("/api/users", post(create_user))
        .route("/api/users/login", post(login_user))
        .route("/api/user", get(get_current_user).put(update_user))
}

#[derive(serde::Serialize, serde::Deserialize)]
struct UserBody<T> {
    user: T,
}

#[derive(serde::Deserialize)]
struct NewUser {
    username: String,
    email: String,
    password: String,
}

#[derive(serde::Deserialize)]
struct LoginUser {
    email: String,
    password: String,
}

#[derive(serde::Deserialize, Default, PartialEq, Eq)]
#[serde(default)]
struct UpdateUser {
    email: Option<String>,
    username: Option<String>,
    password: Option<String>,
    bio: Option<String>,
    image: Option<Uuid>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct User {
    email: String,
    token: String,
    username: String,
    bio: String,
    image: Option<Uuid>,
}

async fn create_user(
    State(ctx): State<ApiContext>,
    Json(req): Json<UserBody<NewUser>>,
) -> Result<Json<UserBody<User>>> {
    let password_hash = hash_password(req.user.password).await?;

    metrics::observe_db_query();
    let user_id = sqlx::query_scalar!(
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

async fn login_user(
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

async fn get_current_user(
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

async fn update_user(
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
            return Err(Error::unprocessable_entity([("image", "invalid profile picture")]));
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

async fn hash_password(password: String) -> Result<String> {
    Ok(tokio::task::spawn_blocking(move || -> Result<String> {
        let salt = SaltString::generate(rand::thread_rng());
        Ok(PasswordHash::generate(Argon2::default(), password, &salt)
            .map_err(|e| anyhow::anyhow!("failed to generate password hash: {}", e))?
            .to_string())
    })
    .await
    .context("panic in generating password hash")??)
}

async fn verify_password(password: String, password_hash: String) -> Result<()> {
    Ok(tokio::task::spawn_blocking(move || -> Result<()> {
        let hash = PasswordHash::new(&password_hash)
            .map_err(|e| anyhow::anyhow!("invalid password hash: {}", e))?;

        hash.verify_password(&[&Argon2::default()], password)
            .map_err(|e| match e {
                argon2::password_hash::Error::Password => Error::Unauthorized,
                _ => anyhow::anyhow!("failed to verify password hash: {}", e).into(),
            })
    })
    .await
    .context("panic in verifying password hash")??)
}
