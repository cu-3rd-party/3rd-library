use axum::extract::State;
use library_application::common::PageRequest;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;
use uuid::Uuid;

use crate::AppState;
use crate::auth::AuthUser;
use crate::error::{ApiResult, ErrorBody};
use crate::extract::{Json, Path, Query};
use crate::routes::submissions::Submission;

const TAG: &str = "moderation";

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(list_moderation_submissions))
        .routes(routes!(moderation_decision))
}

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
    /// `approve` или `reject`.
    pub action: String,
    /// Обязателен для `reject`.
    pub moderator_comment: Option<String>,
}

#[derive(Deserialize, IntoParams, ToSchema)]
pub struct ModerationQuery {
    /// Статус заявки (по умолчанию `pending_review`) или `all`.
    pub status: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

#[utoipa::path(get, path = "/api/moderation/submissions", tag = TAG, params(ModerationQuery),
    responses(
        (status = 200, description = "Moderation submissions list", body = PaginatedModerationResponse),
        (status = 401, description = "Authentication required", body = ErrorBody),
        (status = 403, description = "Moderator role required", body = ErrorBody),
    ),
    security(("bearer_auth" = [])))]
async fn list_moderation_submissions(
    user: AuthUser,
    State(state): State<AppState>,
    Query(query): Query<ModerationQuery>,
) -> ApiResult<Json<PaginatedModerationResponse>> {
    let queue = state
        .moderation
        .queue(
            user.user_id(),
            query.status.as_deref(),
            PageRequest::new(query.page, query.limit),
        )
        .await?;
    let counters = queue.counters;
    Ok(Json(PaginatedModerationResponse {
        items: queue.submissions.items.into_iter().map(Into::into).collect(),
        page: queue.submissions.page,
        limit: queue.submissions.limit,
        total: queue.submissions.total,
        counters: SubmissionStatusCounters {
            all: counters.all,
            draft: counters.draft,
            pending_review: counters.pending_review,
            rejected: counters.rejected,
            approved: counters.approved,
        },
    }))
}

#[utoipa::path(post, path = "/api/moderation/submissions/{submissionId}/decision", tag = TAG,
    params(("submissionId" = Uuid, Path, description = "Submission id")),
    request_body = ModerationDecisionRequest,
    responses(
        (status = 200, description = "Moderation decision applied", body = Submission),
        (status = 400, description = "Invalid action or submission state", body = ErrorBody),
        (status = 401, description = "Authentication required", body = ErrorBody),
        (status = 403, description = "Moderator role required", body = ErrorBody),
        (status = 404, description = "Submission not found", body = ErrorBody),
    ),
    security(("bearer_auth" = [])))]
async fn moderation_decision(
    user: AuthUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(request): Json<ModerationDecisionRequest>,
) -> ApiResult<Json<Submission>> {
    let submission = state
        .moderation
        .decide(user.user_id(), id, &request.action, request.moderator_comment)
        .await?;
    Ok(Json(submission.into()))
}
