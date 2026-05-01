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
    pub image: Option<uuid::Uuid>,
    pub following: bool,
}

async fn get_user_profile(
    maybe_auth_user: MaybeAuthUser,
    State(ctx): State<ApiContext>,
    Path(username): Path<String>,
) -> Result<Json<ProfileBody>> {
    metrics::observe_db_query();
    let row = sqlx::query(
        r#"
            select
                username,
                bio,
                pfp_id,
                exists(
                    select 1 from follow 
                    where followed_user_id = "user".user_id and following_user_id = $2
                ) as following
            from "user"
            where username = $1
        "#,
        username,
        maybe_auth_user.user_id()
    )
    .fetch_optional(&ctx.db)
    .await?
    .ok_or(Error::NotFound)?;

    let profile = Profile {
        username: row.get::<String, _>("username"),
        bio: row.get::<Option<String>, _>("bio").unwrap_or_default(),
        image: row.get::<Option<uuid::Uuid>, _>("pfp_id"),
        following: row.get::<bool, _>("following"),
    };

    Ok(Json(ProfileBody { profile }))
}

async fn follow_user(
    auth_user: AuthUser,
    State(ctx): State<ApiContext>,
    Path(username): Path<String>,
) -> Result<Json<ProfileBody>> {
    let mut tx = ctx.db.begin().await?;

    metrics::observe_db_query();
    let row = sqlx::query(
        r#"select user_id, username, bio, pfp_id from "user" where username = $1"#,
        username
    )
    .fetch_optional(&mut *tx)
    .await?
    .ok_or(Error::NotFound)?;

    metrics::observe_db_query();
    let target_user_id = row.get::<uuid::Uuid, _>("user_id");
    sqlx::query(
        "insert into follow(following_user_id, followed_user_id) values ($1, $2) \
         on conflict do nothing",
        auth_user.user_id,
        target_user_id
    )
    .execute(&mut *tx)
    .await
    .on_constraint("user_cannot_follow_self", |_| Error::Forbidden)?;

    tx.commit().await?;

    Ok(Json(ProfileBody {
        profile: Profile {
            username: row.get::<String, _>("username"),
            bio: row.get::<Option<String>, _>("bio").unwrap_or_default(),
            image: row.get::<Option<uuid::Uuid>, _>("pfp_id"),
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
    let row = sqlx::query(
        r#"select user_id, username, bio, pfp_id from "user" where username = $1"#,
        username
    )
    .fetch_optional(&mut *tx)
    .await?
    .ok_or(Error::NotFound)?;

    metrics::observe_db_query();
    let target_user_id = row.get::<uuid::Uuid, _>("user_id");
    sqlx::query(
        "delete from follow where following_user_id = $1 and followed_user_id = $2",
        auth_user.user_id,
        target_user_id
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(Json(ProfileBody {
        profile: Profile {
            username: row.get::<String, _>("username"),
            bio: row.get::<Option<String>, _>("bio").unwrap_or_default(),
            image: row.get::<Option<uuid::Uuid>, _>("pfp_id"),
            following: false,
        },
    }))
}
