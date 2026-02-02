use crate::http::ApiContext;
use crate::http::error::ResultExt;
use crate::http::extractor::{AuthUser, MaybeAuthUser};
use crate::http::{Error, Result};
use crate::metrics;
use axum::extract::{Path, State};
use axum::routing::{get, post};
use axum::{Json, Router};

pub fn router() -> Router<ApiContext> {
    Router::new()
        .route("/api/profiles/{username}", get(get_user_profile))
        .route(
            "/api/profiles/{username}/follow",
            post(follow_user).delete(unfollow_user),
        )
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct ProfileBody {
    profile: Profile,
}

#[derive(serde::Serialize)]
pub struct Profile {
    pub username: String,
    pub bio: String,
    pub image: Option<String>,
    pub following: bool,
}

async fn get_user_profile(
    maybe_auth_user: MaybeAuthUser,
    State(ctx): State<ApiContext>,
    Path(username): Path<String>,
) -> Result<Json<ProfileBody>> {
    metrics::observe_db_query();
    let profile = sqlx::query_as!(
        Profile,
        r#"
            select
                username,
                bio,
                image,
                exists(
                    select 1 from follow 
                    where followed_user_id = "user".user_id and following_user_id = $2
                ) "following!" -- This tells SQLx that this column will never be null
            from "user"
            where username = $1
        "#,
        username,
        maybe_auth_user.user_id()
    )
    .fetch_optional(&ctx.db)
    .await?
    .ok_or(Error::NotFound)?;

    Ok(Json(ProfileBody { profile }))
}

async fn follow_user(
    auth_user: AuthUser,
    State(ctx): State<ApiContext>,
    Path(username): Path<String>,
) -> Result<Json<ProfileBody>> {
    let mut tx = ctx.db.begin().await?;

    metrics::observe_db_query();
    let user = sqlx::query!(
        r#"select user_id, username, bio, image from "user" where username = $1"#,
        username
    )
    .fetch_optional(&mut *tx)
    .await?
    .ok_or(Error::NotFound)?;

    metrics::observe_db_query();
    sqlx::query!(
        "insert into follow(following_user_id, followed_user_id) values ($1, $2) \
         on conflict do nothing", // If the row already exists, we don't need to do anything.
        auth_user.user_id,
        user.user_id
    )
    .execute(&mut *tx)
    .await
    .on_constraint("user_cannot_follow_self", |_| Error::Forbidden)?;

    tx.commit().await?;

    Ok(Json(ProfileBody {
        profile: Profile {
            username: user.username,
            bio: user.bio,
            image: user.image,
            // We just made sure of this.
            following: true,
        },
    }))
}

async fn unfollow_user(
    auth_user: AuthUser,
    State(ctx): State<ApiContext>,
    Path(username): Path<String>,
) -> Result<Json<ProfileBody>> {
    let mut tx = ctx.db.begin().await?;

    metrics::observe_db_query();
    let user = sqlx::query!(
        r#"select user_id, username, bio, image from "user" where username = $1"#,
        username
    )
    .fetch_optional(&mut *tx)
    .await?
    .ok_or(Error::NotFound)?;

    metrics::observe_db_query();
    sqlx::query!(
        "delete from follow where following_user_id = $1 and followed_user_id = $2",
        auth_user.user_id,
        user.user_id
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(Json(ProfileBody {
        profile: Profile {
            username: user.username,
            bio: user.bio,
            image: user.image,
            following: false,
        },
    }))
}
