mod create;
mod get;
mod helpers;
mod login;
mod models;
mod update;

use crate::http::ApiContext;
use axum::Router;
use axum::routing::{get, post};

use create::create_user;
use get::get_current_user;
use login::login_user;
use update::update_user;

pub fn router() -> Router<ApiContext> {
    Router::new()
        .route("/api/users", post(create_user))
        .route("/api/users/login", post(login_user))
        .route("/api/user", get(get_current_user).put(update_user))
}
