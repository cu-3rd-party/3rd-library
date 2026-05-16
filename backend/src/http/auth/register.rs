use crate::http::{ApiContext, Result};
use crate::metrics;
#[cfg(feature = "smtp")]
use crate::smtp;
use axum::Json;
use axum::extract::State;
use std::sync::OnceLock;

use super::models::*;
use crate::http::error::{Error, ResultExt};
use crate::http::users::hash_password;
use rand::Rng;
use regex::Regex;

const VERIFICATION_CODE_LENGTH: usize = 6;
static EMAIL_REGEX: OnceLock<Regex> = OnceLock::new();

#[utoipa::path(
    post,
    path = "/api/auth/register",
    tag = "auth",
    request_body = RegisterRequest,
    responses(
        (status = 200, description = "User registered successfully", body = RegisterResponse),
        (status = 400, description = "Invalid request", body = crate::http::error::ApiError),
        (status = 409, description = "Email already exists", body = crate::http::error::ApiError),
        (status = 500, description = "Internal server error", body = crate::http::error::ApiError)
    )
)]
pub async fn register_user(
    State(ctx): State<ApiContext>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<RegisterResponse>> {
    let email_regex = EMAIL_REGEX
        .get_or_init(|| Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap());
    if !email_regex.is_match(&req.email) {
        return Err(Error::BadRequest);
    }

    let password_hash = hash_password(req.password).await?;

    let code: String = (0..VERIFICATION_CODE_LENGTH)
        .map(|_| {
            let idx = rand::thread_rng().gen_range(0..10);
            char::from_digit(idx as u32, 10).unwrap()
        })
        .collect();

    let code_expires_at = chrono::Utc::now() + chrono::Duration::hours(24);

    metrics::observe_db_query();
    let is_email_verified: Option<bool> = sqlx::query_scalar(
        r#"select is_email_verified from web_user where email == $1 limit 1 returning is_email_verified"#,
    )
        .bind(&req.email)
        .fetch_one(&ctx.db)
        .await.ok();
    if let Some(is_email_verified) = is_email_verified {
        if !is_email_verified {
            return Err(Error::Conflict("email not verified".to_string()));
        }
    }

    metrics::observe_db_query();
    let user_id: uuid::Uuid = sqlx::query_scalar(
        r#"insert into web_user (email, password_hash, name, verification_code, verification_code_issued_at, verification_code_expires_at)
           values ($1, $2, $3, $4, $5, $6)
           returning user_id"#
    )
    .bind(&req.email)
    .bind(&password_hash)
    .bind(&req.name)
    .bind(&code)
    .bind(chrono::Utc::now())
    .bind(code_expires_at)
    .fetch_one(&ctx.db)
    .await
    .on_constraint("web_user_email_key", |_| {
        Error::conflict("email_already_exists", "Email already exists")
    })?;

    log::info!(
        "Registered user {} with verification code {}",
        user_id,
        code
    );

    metrics::record_user_created();

    let user = WebUser {
        id: user_id,
        name: req.name.clone(),
        email: req.email,
        bio: String::new(),
        is_email_verified: false,
        can_submit_materials: false,
        roles: vec!["user".to_string()],
    };

    #[cfg(feature = "smtp")]
    tokio::spawn(smtp::send_verification_code(
        ctx,
        user.email.clone(),
        code.clone(),
    ));

    Ok(Json(RegisterResponse {
        user,
        verification_required: true,
        verification_channel: Some("email_code".to_string()),
    }))
}
