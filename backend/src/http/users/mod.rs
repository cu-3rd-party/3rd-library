pub(crate) mod get;
pub mod helpers;
pub(crate) mod models;
pub(crate) mod update;

pub use helpers::*;

use crate::http::ApiContext;
use axum::Router;
use axum::routing::{get, patch};

use get::{get_current_user, get_user_by_id, get_users};
use update::update_user_profile;

pub fn router() -> Router<ApiContext> {
    Router::new()
        .route("/api/users/me", get(get_current_user))
        .route("/api/users", get(get_users))
        .route("/api/users/{userId}", get(get_user_by_id))
        .route("/api/users/me", patch(update_user_profile))
}
