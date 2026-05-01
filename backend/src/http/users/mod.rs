mod get;
pub mod helpers;
mod models;
mod update;

pub use helpers::*;

use crate::http::ApiContext;
use axum::Router;
use axum::routing::{get, patch};

use get::{get_current_user, get_user_by_id, get_users};
use update::update_user_profile;

pub fn router() -> Router<ApiContext> {
    Router::new()
        .route("/users/me", get(get_current_user))
        .route("/users", get(get_users))
        .route("/users/{userId}", get(get_user_by_id))
        .route("/users/me", patch(update_user_profile))
}
