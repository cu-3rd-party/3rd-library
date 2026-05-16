pub(crate) mod decision;
pub(crate) mod list;
pub(crate) mod models;

use crate::http::ApiContext;
use axum::Router;
use axum::routing::{get, post};

use decision::moderation_decision;
use list::list_moderation_submissions;

pub fn router() -> Router<ApiContext> {
    Router::new()
        .route("/api/moderation/submissions", get(list_moderation_submissions))
        .route(
            "/api/moderation/submissions/{submissionId}/decision",
            post(moderation_decision),
        )
}
