pub(crate) mod create;
pub(crate) mod get;
pub(crate) mod get_single;
mod helpers;
pub(crate) mod list;
pub mod models;
pub(crate) mod update;

use axum::extract::DefaultBodyLimit;
pub use models::*;

use crate::http::ApiContext;
use axum::Router;
use axum::routing::get;
use create::create_submission;
use get::get_material;
use get_single::get_submission_by_id;
use list::{list_materials, list_submissions};
use update::update_submission;

pub fn router() -> Router<ApiContext> {
    Router::new()
        .route("/api/materials", get(list_materials))
        .route("/api/materials/{materialId}", get(get_material))
        .route(
            "/api/materials/submissions",
            get(list_submissions).post(create_submission),
        )
        .route(
            "/api/materials/submissions/{submissionId}",
            get(get_submission_by_id).patch(update_submission),
        )
        .layer(DefaultBodyLimit::max(30 * 1024 * 1024 * 1024))
}
