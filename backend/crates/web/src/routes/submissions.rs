use axum::extract::multipart::Field;
use axum::extract::{DefaultBodyLimit, State};
use library_application::common::{PageDto, PageRequest};
use library_application::submissions::{CreateSubmission, NewFile, SubmissionDto, UpdateSubmission};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;
use uuid::Uuid;

use crate::AppState;
use crate::auth::{AuthUser, VerifiedUser};
use crate::error::{ApiError, ApiResult, ErrorBody};
use crate::extract::{Json, Multipart, Path, Query};
use crate::routes::materials::{MaterialFile, TAG};
use crate::routes::timestamp;

/// Лимит тела запроса с файлами (как в прежнем бэкенде).
const MAX_UPLOAD_BYTES: usize = 30 * 1024 * 1024 * 1024;

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(list_submissions, create_submission))
        .routes(routes!(get_submission_by_id, update_submission))
        .layer(DefaultBodyLimit::max(MAX_UPLOAD_BYTES))
}

#[derive(Serialize, ToSchema)]
pub struct Submission {
    pub id: Uuid,
    pub files: Vec<MaterialFile>,
    pub status: String,
    pub moderator_comment: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub submitted_at: Option<String>,
    pub reviewed_at: Option<String>,
    pub published_at: Option<String>,
}

impl From<SubmissionDto> for Submission {
    fn from(dto: SubmissionDto) -> Self {
        Self {
            id: dto.id,
            files: dto.files.into_iter().map(Into::into).collect(),
            status: dto.status.as_str().to_owned(),
            moderator_comment: Some(dto.moderator_comment),
            created_at: timestamp(dto.created),
            updated_at: timestamp(dto.last_modified),
            submitted_at: dto.submitted_at.map(timestamp),
            reviewed_at: dto.reviewed_at.map(timestamp),
            published_at: dto.published_at.map(timestamp),
        }
    }
}

#[derive(Serialize, ToSchema)]
pub struct PaginatedSubmissionsResponse {
    pub items: Vec<Submission>,
    pub page: i64,
    pub limit: i64,
    pub total: i64,
}

impl From<PageDto<SubmissionDto>> for PaginatedSubmissionsResponse {
    fn from(page: PageDto<SubmissionDto>) -> Self {
        Self {
            items: page.items.into_iter().map(Into::into).collect(),
            page: page.page,
            limit: page.limit,
            total: page.total,
        }
    }
}

#[derive(Deserialize, IntoParams, ToSchema)]
pub struct ListSubmissionsQuery {
    /// Статус заявки или `all`.
    pub status: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

/// JSON-часть `data` в multipart-запросе создания заявки.
#[derive(Deserialize, ToSchema)]
pub struct CreateSubmissionRequest {
    pub title: String,
    pub description: Option<String>,
    pub courses: Vec<String>,
    pub subjects: Vec<String>,
    pub r#type: String,
    pub difficulty: String,
}

/// Поля multipart-запроса правки заявки (все необязательные) и части `files`.
#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSubmissionRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    /// JSON-массив строк.
    pub courses: Option<String>,
    /// JSON-массив строк.
    pub subjects: Option<String>,
    pub r#type: Option<String>,
    pub difficulty: Option<String>,
    /// JSON-массив id файлов, которые нужно оставить.
    pub keep_file_ids: Option<String>,
}

#[utoipa::path(get, path = "/api/materials/submissions", tag = TAG, params(ListSubmissionsQuery),
    responses(
        (status = 200, description = "Current user submissions", body = PaginatedSubmissionsResponse),
        (status = 401, description = "Authentication required", body = ErrorBody),
    ),
    security(("bearer_auth" = [])))]
async fn list_submissions(
    user: AuthUser,
    State(state): State<AppState>,
    Query(query): Query<ListSubmissionsQuery>,
) -> ApiResult<Json<PaginatedSubmissionsResponse>> {
    let page = PageRequest::new(query.page, query.limit);
    let submissions = state
        .submissions
        .list_own(user.user_id(), query.status.as_deref(), page)
        .await?;
    Ok(Json(submissions.into()))
}

#[utoipa::path(post, path = "/api/materials/submissions", tag = TAG,
    request_body(content = String, content_type = "multipart/form-data",
        description = "Multipart form with `data` JSON part (CreateSubmissionRequest) and `files` parts"),
    responses(
        (status = 200, description = "Submission created", body = Submission),
        (status = 400, description = "Invalid multipart payload", body = ErrorBody),
        (status = 401, description = "Verified authentication required", body = ErrorBody),
    ),
    security(("bearer_auth" = [])))]
async fn create_submission(
    user: VerifiedUser,
    State(state): State<AppState>,
    Multipart(mut multipart): Multipart,
) -> ApiResult<Json<Submission>> {
    let mut data: Option<CreateSubmissionRequest> = None;
    let mut files = Vec::new();
    while let Some(field) = multipart.next_field().await? {
        match field.name().unwrap_or_default() {
            "data" => data = Some(json(&field.text().await?)?),
            "files" => files.push(new_file(field).await?),
            other => return Err(ApiError::bad_request(format!("unexpected multipart field '{other}'"))),
        }
    }
    let data = data.ok_or_else(|| ApiError::bad_request("multipart field 'data' is required"))?;

    let submission = state
        .submissions
        .create(
            user.user_id(),
            CreateSubmission {
                title: data.title,
                description: data.description,
                courses: data.courses,
                subjects: data.subjects,
                kind: data.r#type,
                difficulty: data.difficulty,
                files,
            },
        )
        .await?;
    Ok(Json(submission.into()))
}

#[utoipa::path(get, path = "/api/materials/submissions/{submissionId}", tag = TAG,
    params(("submissionId" = Uuid, Path, description = "Submission id")),
    responses(
        (status = 200, description = "Submission details", body = Submission),
        (status = 401, description = "Authentication required", body = ErrorBody),
        (status = 403, description = "Access denied", body = ErrorBody),
        (status = 404, description = "Submission not found", body = ErrorBody),
    ),
    security(("bearer_auth" = [])))]
async fn get_submission_by_id(
    user: AuthUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<Submission>> {
    Ok(Json(state.submissions.get(user.user_id(), id).await?.into()))
}

#[utoipa::path(patch, path = "/api/materials/submissions/{submissionId}", tag = TAG,
    params(("submissionId" = Uuid, Path, description = "Submission id")),
    request_body(content = UpdateSubmissionRequest, content_type = "multipart/form-data",
        description = "Multipart form fields for submission updates and optional `files` parts"),
    responses(
        (status = 200, description = "Submission updated", body = Submission),
        (status = 400, description = "Invalid request", body = ErrorBody),
        (status = 401, description = "Verified authentication required", body = ErrorBody),
        (status = 403, description = "Access denied or invalid submission status", body = ErrorBody),
        (status = 404, description = "Submission not found", body = ErrorBody),
    ),
    security(("bearer_auth" = [])))]
async fn update_submission(
    user: VerifiedUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Multipart(mut multipart): Multipart,
) -> ApiResult<Json<Submission>> {
    let mut request = UpdateSubmission::default();
    while let Some(field) = multipart.next_field().await? {
        match field.name().unwrap_or_default() {
            "title" => request.title = Some(field.text().await?),
            "description" => request.description = Some(field.text().await?),
            "courses" => request.courses = Some(json(&field.text().await?)?),
            "subjects" => request.subjects = Some(json(&field.text().await?)?),
            "type" => request.kind = Some(field.text().await?),
            "difficulty" => request.difficulty = Some(field.text().await?),
            "keepFileIds" => request.keep_file_ids = Some(json(&field.text().await?)?),
            "files" => request.files.push(new_file(field).await?),
            _ => {}
        }
    }

    Ok(Json(
        state.submissions.update(user.user_id(), id, request).await?.into(),
    ))
}

fn json<T: DeserializeOwned>(value: &str) -> ApiResult<T> {
    serde_json::from_str(value).map_err(|error| ApiError::bad_request(format!("invalid JSON field: {error}")))
}

async fn new_file(field: Field<'_>) -> ApiResult<NewFile> {
    let file_name = field
        .file_name()
        .ok_or_else(|| ApiError::bad_request("file name is required"))?
        .to_owned();
    let content_type = field.content_type().map(str::to_owned);
    Ok(NewFile {
        file_name,
        content_type,
        content: field.bytes().await?,
    })
}
