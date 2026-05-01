use crate::http::{ApiContext, Result};
use axum::Json;
use axum::extract::State;

use crate::http::error::Error;
use rand::Rng;

use super::models::*;

pub async fn resend_verification_code(
    State(ctx): State<ApiContext>,
    Json(req): Json<ResendVerificationCodeRequest>,
) -> Result<Json<()>> {
    let code: String = (0..6)
        .map(|_| {
            let idx = rand::thread_rng().gen_range(0..10);
            char::from_digit(idx as u32, 10).unwrap()
        })
        .collect();

    let code_expires_at = time::OffsetDateTime::now_utc() + time::Duration::hours(24);

    let result = sqlx::query(
        "update web_user set verification_code = $1, verification_code_expires_at = $2 where email = $3 and not is_email_verified"
    )
    .bind(&code)
    .bind(code_expires_at)
    .bind(&req.email)
    .execute(&ctx.db)
    .await?;

    if result.rows_affected() == 0 {
        return Err(Error::NotFound);
    }

    log::info!("Resent verification code to {}", req.email);

    Ok(Json(()))
}
