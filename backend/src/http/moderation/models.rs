use serde::Serialize;

use crate::http::materials::models::Submission;

#[derive(Serialize)]
pub struct SubmissionStatusCounters {
    pub all: i64,
    pub draft: i64,
    pub pending_review: i64,
    pub rejected: i64,
    pub approved: i64,
}

#[derive(Serialize)]
pub struct PaginatedModerationResponse {
    pub items: Vec<Submission>,
    pub page: i64,
    pub limit: i64,
    pub total: i64,
    pub counters: SubmissionStatusCounters,
}

#[derive(serde::Deserialize)]
pub struct ModerationDecisionRequest {
    pub action: String,
    pub moderator_comment: Option<String>,
}
