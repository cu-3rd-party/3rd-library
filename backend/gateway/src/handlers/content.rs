use axum::{
    Json,
    body::Body,
    extract::{Multipart, Path, Query, State},
    http::{HeaderMap, StatusCode, header},
    response::Response,
};

use crate::errors::ApiError;
use crate::handlers::{SharedState, anonymous_id, bearer_token};
use crate::models::{ContentListResponse, ContentMetadata, StatsResponse};
use crate::proto::{auth, content, engagement, statistics};

#[derive(Debug, serde::Deserialize)]
pub struct ListQuery {
    page_size: Option<i32>,
    page_token: Option<String>,
}

pub async fn list_contents(
    State(state): State<SharedState>,
    Query(params): Query<ListQuery>,
) -> Result<Json<ContentListResponse>, ApiError> {
    let mut client = state.clients.content_client();
    let response = client
        .list(content::ListRequest {
            page_size: params.page_size.unwrap_or(20),
            page_token: params.page_token.unwrap_or_default(),
        })
        .await
        .map_err(ApiError::from)?;

    let result = response.into_inner();
    let items = result
        .items
        .into_iter()
        .map(to_content_metadata)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(Json(ContentListResponse {
        items,
        next_page_token: result.next_page_token,
    }))
}

pub async fn get_content(
    State(state): State<SharedState>,
    Path(content_id): Path<String>,
    headers: HeaderMap,
) -> Result<Response, ApiError> {
    let (file_bytes, filename) = fetch_content(&state, &content_id).await?;
    let user_id = resolve_user_id(&state, &headers, false).await?;
    record_interaction(&state, &content_id, &user_id, InteractionKind::View).await;

    Ok(file_response(file_bytes, &filename, true))
}

pub async fn download_content(
    State(state): State<SharedState>,
    Path(content_id): Path<String>,
    headers: HeaderMap,
) -> Result<Response, ApiError> {
    let (file_bytes, filename) = fetch_content(&state, &content_id).await?;
    let user_id = resolve_user_id(&state, &headers, false).await?;
    record_interaction(&state, &content_id, &user_id, InteractionKind::Download).await;

    Ok(file_response(file_bytes, &filename, false))
}

pub async fn upload_content(
    State(state): State<SharedState>,
    headers: HeaderMap,
    mut multipart: Multipart,
) -> Result<Json<ContentMetadata>, ApiError> {
    let token = bearer_token(&headers)?;
    let user = validate_user(&state, &token).await?;

    let mut title = None;
    let mut description = None;
    let mut filename = None;
    let mut file_bytes = Vec::new();

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|err| ApiError::BadRequest(format!("failed to read multipart field: {err}")))?
    {
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
                description =
                    Some(field.text().await.map_err(|err| {
                        ApiError::BadRequest(format!("invalid description: {err}"))
                    })?);
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

    let mut client = state.clients.content_client();
    let response = client
        .upload(content::UploadRequest {
            owner_id: user.id,
            title,
            description,
            filename: filename.clone(),
            file_bytes,
        })
        .await
        .map_err(ApiError::from)?;

    let result = response.into_inner();
    let content = result
        .content
        .ok_or_else(|| ApiError::Upstream("missing content payload".to_string()))?;

    Ok(Json(to_content_metadata(content)?))
}

pub async fn get_stats(
    State(state): State<SharedState>,
    Path(content_id): Path<String>,
) -> Result<Json<StatsResponse>, ApiError> {
    let mut client = state.clients.statistics_client();
    let response = client
        .get_content_stats(statistics::GetContentStatsRequest { content_id })
        .await
        .map_err(ApiError::from)?;
    let stats = response
        .into_inner()
        .stats
        .ok_or_else(|| ApiError::Upstream("missing stats payload".to_string()))?;
    Ok(Json(StatsResponse {
        views: stats.views,
        downloads: stats.downloads,
        unique_users: stats.unique_users,
        last_interaction_at: stats.last_interaction_at,
    }))
}

async fn fetch_content(
    state: &SharedState,
    content_id: &str,
) -> Result<(Vec<u8>, String), ApiError> {
    let mut client = state.clients.content_client();
    let response = client
        .get(content::GetRequest {
            content_id: content_id.to_string(),
        })
        .await
        .map_err(ApiError::from)?;
    let result = response.into_inner();
    let content = result
        .content
        .ok_or_else(|| ApiError::Upstream("missing content payload".to_string()))?;
    Ok((result.file_bytes, content.filename))
}

fn file_response(file_bytes: Vec<u8>, filename: &str, inline: bool) -> Response {
    let disposition = if inline { "inline" } else { "attachment" };
    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/pdf")
        .header(
            header::CONTENT_DISPOSITION,
            format!("{disposition}; filename=\"{filename}\""),
        )
        .body(Body::from(file_bytes))
        .expect("response build");
    response
}

async fn validate_user(state: &SharedState, token: &str) -> Result<auth::User, ApiError> {
    let mut client = state.clients.auth_client();
    let response = client
        .validate(auth::ValidateRequest {
            token: token.to_string(),
        })
        .await
        .map_err(ApiError::from)?;
    response
        .into_inner()
        .user
        .ok_or_else(|| ApiError::Unauthorized("invalid token".to_string()))
}

async fn resolve_user_id(
    state: &SharedState,
    headers: &HeaderMap,
    require_auth: bool,
) -> Result<String, ApiError> {
    if let Ok(token) = bearer_token(headers) {
        let user = validate_user(state, &token).await?;
        return Ok(user.id);
    }
    if require_auth {
        return Err(ApiError::Unauthorized("missing bearer token".to_string()));
    }
    Ok(anonymous_id(headers))
}

enum InteractionKind {
    View,
    Download,
}

async fn record_interaction(
    state: &SharedState,
    content_id: &str,
    user_id: &str,
    kind: InteractionKind,
) {
    let (engagement_type, statistics_type) = match kind {
        InteractionKind::View => (
            engagement::InteractionType::View as i32,
            statistics::InteractionType::View as i32,
        ),
        InteractionKind::Download => (
            engagement::InteractionType::Download as i32,
            statistics::InteractionType::Download as i32,
        ),
    };

    let mut engagement_client = state.clients.engagement_client();
    let mut statistics_client = state.clients.statistics_client();
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;

    if let Err(err) = engagement_client
        .record_interaction(engagement::RecordInteractionRequest {
            content_id: content_id.to_string(),
            user_id: user_id.to_string(),
            r#type: engagement_type,
            occurred_at: now,
        })
        .await
    {
        tracing::warn!(error = %err, "failed to record engagement");
    }

    if let Err(err) = statistics_client
        .record_interaction(statistics::RecordInteractionRequest {
            content_id: content_id.to_string(),
            user_id: user_id.to_string(),
            r#type: statistics_type,
            occurred_at: now,
        })
        .await
    {
        tracing::warn!(error = %err, "failed to record statistics");
    }
}

fn to_content_metadata(content: content::Content) -> Result<ContentMetadata, ApiError> {
    Ok(ContentMetadata {
        id: content.id,
        owner_id: content.owner_id,
        title: content.title,
        description: content.description,
        filename: content.filename,
        size_bytes: content.size_bytes,
        created_at: content.created_at,
    })
}
