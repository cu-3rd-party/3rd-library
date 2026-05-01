mod login;
mod logout;
mod refresh;
mod register;
mod resend_verification;
mod verify_email;

pub mod models;

use crate::http::ApiContext;
use axum::Router;
use axum::routing::post;

pub use login::login_user;
pub use logout::logout;
pub use refresh::refresh_token;
pub use register::register_user;
pub use resend_verification::resend_verification_code;
pub use verify_email::verify_email;

pub fn router() -> Router<ApiContext> {
    Router::new()
        .route("/auth/register", post(register_user))
        .route("/auth/verify-email", post(verify_email))
        .route(
            "/auth/resend-verification-code",
            post(resend_verification_code),
        )
        .route("/auth/login", post(login_user))
        .route("/auth/refresh", post(refresh_token))
        .route("/auth/logout", post(logout))
}
