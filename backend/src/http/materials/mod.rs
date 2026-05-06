mod create;
mod get;
mod get_single;
mod list;
pub mod models;
mod update;
mod helpers;

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
        .route("/materials", get(list_materials))
        .route("/materials/{materialId}", get(get_material))
        .route(
            "/materials/submissions",
            get(list_submissions).post(create_submission),
        )
        .route(
            "/materials/submissions/{submissionId}",
            get(get_submission_by_id).patch(update_submission),
        )
}
