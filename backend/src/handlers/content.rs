use axum::{
    Json,
    body::Body,
    extract::{Multipart, Path, Query, State},
    http::{HeaderMap, StatusCode, header},
    response::Response,
};

use crate::auth::{AuthError, User};
use crate::content::{ContentError, ContentItem};
use crate::errors::ApiError;
use crate::handlers::{SharedState, anonymous_id, bearer_token};
use crate::models::{ContentListResponse, ContentMetadata, StatsResponse};
use crate::statistics::InteractionType;

#[derive(Debug, serde::Deserialize)]
pub struct ListQuery {
    page_size: Option<i32>,
    page_token: Option<String>,
}

pub async fn list_contents(
    State(state): State<SharedState>,
    Query(params): Query<ListQuery>,
) -> Result<Json<ContentListResponse>, ApiError> {
    let (items, next_page_token) = state
        .content
        .list_items(params.page_size.unwrap_or(20), params.page_token.unwrap_or_default())
        .await
        .map_err(map_content_error)?;
    let items = items.into_iter().map(to_content_metadata).collect();
    Ok(Json(ContentListResponse { items, next_page_token }))
}

pub async fn get_content(
    State(state): State<SharedState>,
    Path(content_id): Path<String>,
    headers: HeaderMap,
) -> Result<Response, ApiError> {
    let (item, file_bytes) = state.content.get(content_id.clone()).await.map_err(map_content_error)?;
    let user_id = resolve_user_id(&state, &headers, false).await?;
    record_interaction(&state, &content_id, &user_id, InteractionType::View).await;
    Ok(file_response(file_bytes, &item.filename, true))
}

pub async fn download_content(
    State(state): State<SharedState>,
    Path(content_id): Path<String>,
    headers: HeaderMap,
) -> Result<Response, ApiError> {
    let (item, file_bytes) = state.content.get(content_id.clone()).await.map_err(map_content_error)?;
    let user_id = resolve_user_id(&state, &headers, false).await?;
    record_interaction(&state, &content_id, &user_id, InteractionType::Download).await;
    Ok(file_response(file_bytes, &item.filename, false))
}

pub async fn upload_content(
    State(state): State<SharedState>,
    headers: HeaderMap,
    mut multipart: Multipart,
) -> Result<Json<ContentMetadata>, ApiError> {
    let token = bearer_token(&headers).ok_or_else(|| ApiError::Unauthorized("missing bearer token".to_string()))?;
    let user = validate_user(&state, &token).await?;

    let mut title = None;
    let mut description = None;
    let mut filename = None;
    let mut file_bytes = Vec::new();

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|err| ApiError::BadRequest(format!("failed to read multipart field: {err}")))? {
        let name = field.name().unwrap_or("");
        match name {
            "title" => {
                title = Some(
                    field
                        .text()
                        .await
                        .map_err(|err| ApiError::BadRequest(format!("invalid title: {err}")))?,
                );
            }
            "description" => {
                description = Some(
                    field
                        .text()
                        .await
                        .map_err(|err| ApiError::BadRequest(format!("invalid description: {err}")))?,
                );
            }
            "file" => {
                filename = field.file_name().map(|value| value.to_string());
                let data = field
                    .bytes()
                    .await
                    .map_err(|err| ApiError::BadRequest(format!("invalid file payload: {err}")))?;
                if data.len() > state.max_upload_bytes {
                    return Err(ApiError::BadRequest("file too large".to_string()));
                }
                file_bytes = data.to_vec();
            }
            _ => {}
        }
    }

    let title = title.unwrap_or_default();
    let description = description.unwrap_or_default();
    let filename = filename.unwrap_or_else(|| "upload.pdf".to_string());

    let item = state
        .content
        .upload(
            user.id,
            title,
            description,
            filename.clone(),
            file_bytes,
        )
        .await
        .map_err(map_content_error)?;

    Ok(Json(to_content_metadata(item)))
}

pub async fn get_stats(
    State(state): State<SharedState>,
    Path(content_id): Path<String>,
) -> Result<Json<StatsResponse>, ApiError> {
    let stats = state
        .statistics
        .stats_for(content_id)
        .await
        .map_err(map_statistics_error)?;
    Ok(Json(StatsResponse {
        views: stats.views,
        downloads: stats.downloads,
        unique_users: stats.unique_users,
        last_interaction_at: stats.last_interaction_at,
    }))
}

fn file_response(file_bytes: Vec<u8>, filename: &str, inline: bool) -> Response {
    let disposition = if inline { "inline" } else { "attachment" };
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/pdf")
        .header(
            header::CONTENT_DISPOSITION,
            format!("{disposition}; filename=\"{filename}\""),
        )
        .body(Body::from(file_bytes))
        .expect("response build")
}

async fn validate_user(state: &SharedState, token: &str) -> Result<User, ApiError> {
    state.auth.validate(token).await.map_err(map_auth_error)
}

async fn resolve_user_id(
    state: &SharedState,
    headers: &HeaderMap,
    require_auth: bool,
) -> Result<String, ApiError> {
    if let Some(token) = bearer_token(headers) {
        let user = validate_user(state, &token).await?;
        return Ok(user.id);
    }
    if require_auth {
        return Err(ApiError::Unauthorized("missing bearer token".to_string()));
    }
    Ok(anonymous_id(headers))
}

async fn record_interaction(
    state: &SharedState,
    content_id: &str,
    user_id: &str,
    kind: InteractionType,
) {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;

    if let Err(err) = state
        .statistics
        .record(content_id.to_string(), user_id.to_string(), kind, now)
        .await
    {
        tracing::warn!(error = ?err, "failed to record statistics");
    }
}

fn to_content_metadata(item: ContentItem) -> ContentMetadata {
    ContentMetadata {
        id: item.id,
        owner_id: item.owner_id,
        title: item.title,
        description: item.description,
        filename: item.filename,
        size_bytes: item.size_bytes,
        created_at: item.created_at,
    }
}

fn map_content_error(err: ContentError) -> ApiError {
    match err {
        ContentError::InvalidInput(msg) => ApiError::BadRequest(msg),
        ContentError::NotFound => ApiError::NotFound("content not found".to_string()),
        ContentError::Io(msg) => ApiError::Internal(msg),
        ContentError::Db(msg) => ApiError::Internal(msg),
    }
}

fn map_statistics_error(err: crate::statistics::StatisticsError) -> ApiError {
    match err {
        crate::statistics::StatisticsError::InvalidInput(msg) => ApiError::BadRequest(msg),
        crate::statistics::StatisticsError::Db(msg) => ApiError::Internal(msg),
    }
}

fn map_auth_error(err: AuthError) -> ApiError {
    match err {
        AuthError::InvalidInput => ApiError::BadRequest("invalid input".to_string()),
        AuthError::InvalidCredentials => ApiError::Unauthorized("invalid credentials".to_string()),
        AuthError::EmailTaken => ApiError::Conflict("email already registered".to_string()),
        AuthError::UserNotFound | AuthError::TokenInvalid => {
            ApiError::Unauthorized("invalid token".to_string())
        }
        AuthError::Internal(msg) => ApiError::Internal(msg),
    }
}
