use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::http::materials::models::Submission;

#[derive(Serialize, ToSchema)]
pub struct SubmissionStatusCounters {
    pub all: i64,
    pub draft: i64,
    pub pending_review: i64,
    pub rejected: i64,
    pub approved: i64,
}

#[derive(Serialize, ToSchema)]
pub struct PaginatedModerationResponse {
    pub items: Vec<Submission>,
    pub page: i64,
    pub limit: i64,
    pub total: i64,
    pub counters: SubmissionStatusCounters,
}

#[derive(Deserialize, ToSchema)]
pub struct ModerationDecisionRequest {
    pub action: String,
    pub moderator_comment: Option<String>,
}

#[derive(Deserialize, IntoParams, ToSchema)]
pub struct ModerationQuery {
    pub status: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}
