use crate::http::{ApiContext, Result};
use crate::metrics;
use axum::Json;
use axum::extract::State;

use crate::http::error::Error;
use crate::http::extractor::AuthUser;
use sqlx::{Executor, Row};
use uuid::Uuid;

use super::models::*;
use crate::http::users::verify_password;

pub async fn login_user(
    State(ctx): State<ApiContext>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<AuthResponse>> {
    metrics::observe_db_query();
    let row = sqlx::query(
        r#"
            select user_id, email, name, bio, roles, is_email_verified, password_hash
            from web_user where email = $1
        "#,
    )
    .bind(&req.email)
    .fetch_optional(&ctx.db)
    .await?
    .ok_or(Error::unauthorized(
        "invalid_credentials",
        "Invalid email or password",
    ))?;

    verify_password(req.password, row.get::<String, _>("password_hash")).await?;

    if !row.get::<bool, _>("is_email_verified") {
        return Err(Error::forbidden(
            "email_not_verified",
            "Email is not verified",
        ));
    }

    let user_id = row.get::<uuid::Uuid, _>("user_id");
    let auth_user = AuthUser {
        user_id,
        session_id: Uuid::new_v4(),
    };

    let roles: Vec<String> = row
        .get::<Option<Vec<String>>, _>("roles")
        .unwrap_or_else(|| vec!["user".to_string()]);

    let web_user = WebUser {
        id: user_id,
        name: row.get::<String, _>("name"),
        email: row.get::<String, _>("email"),
        bio: row.get::<Option<String>, _>("bio").unwrap_or_default(),
        is_email_verified: true,
        can_submit_materials: true,
        roles,
    };

    let tokens = TokenPair {
        access_token: auth_user.to_jwt(&ctx).await?,
        refresh_token: auth_user.session_id.to_string(),
        expires_in: 1209600,
    };

    Ok(Json(AuthResponse {
        user: web_user,
        tokens,
    }))
}
