mod get;
mod create;
mod decision;

pub use models::*;

use crate::http::ApiContext;
use axum::Router;
use axum::routing::{get, post, patch};

use get::get_submission;
use create::create_submission_handler;
use decision::moderation_decision;

pub fn router() -> Router<ApiContext> {
    Router::new()
        .route("/materials/submissions", get(list_submissions_handler).post(create_submission_handler))
        .route("/materials/submissions/{submissionId}", get(get_submission))
        .merge(super::moderation::router())
}