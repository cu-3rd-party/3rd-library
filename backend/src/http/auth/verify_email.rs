use crate::http::{ApiContext, Result};
use crate::metrics;
use axum::Json;
use axum::extract::State;

use crate::http::error::Error;
use crate::http::extractor::AuthUser;
use chrono::{DateTime, Utc};
use sqlx::Row;
use uuid::Uuid;

use super::models::*;

pub async fn verify_email(
    State(ctx): State<ApiContext>,
    Json(req): Json<VerifyEmailRequest>,
) -> Result<Json<AuthResponse>> {
    metrics::observe_db_query();
    let row = sqlx::query(
        r#"
            select user_id, email, name, bio, roles, is_email_verified, password_hash, verification_code, verification_code_expires_at
            from web_user where email = $1
        "#
    )
    .bind(&req.email)
    .fetch_optional(&ctx.db)
    .await?
    .ok_or(Error::NotFound)?;

    if row.get::<bool, _>("is_email_verified") {
        return Err(Error::conflict(
            "email_already_verified",
            "Email already verified",
        ));
    }

    if row.get::<Option<String>, _>("verification_code") != Some(req.code) {
        return Err(Error::BadRequest);
    }

    let code_expires_at: Option<DateTime<Utc>> =
        row.get::<Option<DateTime<Utc>>, _>("verification_code_expires_at");
    let now = Utc::now();
    if let Some(expires) = code_expires_at {
        if expires < now {
            return Err(Error::bad_request(
                "verification_code_expired",
                "Verification code has expired",
            ));
        }
    }

    metrics::observe_db_query();
    let user_id = row.get::<Uuid, _>("user_id");
    sqlx::query(
        "update web_user set is_email_verified = true, verification_code = null, verification_code_expires_at = null where user_id = $1"
    )
    .bind(user_id)
    .execute(&ctx.db)
    .await?;

    let auth_user = AuthUser {
        user_id,
        session_id: Uuid::new_v4(),
        verified: true, // сюда мы доходим только если query выше удался, так что хардкодить можно
    };

    let roles: Vec<String> = row
        .get::<Option<Vec<String>>, _>("roles")
        .unwrap_or_else(|| vec!["user".to_string()]);

    let web_user = WebUser {
        id: user_id,
        name: row.get::<String, _>("name"),
        email: row.get::<String, _>("email"),
        bio: row.get::<Option<String>, _>("bio").unwrap_or_default(),
        is_email_verified: true, // Сейм
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
