use axum::{
    Json,
    extract::{Path, State},
    http::{HeaderMap, header},
};

use crate::errors::ApiError;
use crate::handlers::{SharedState, bearer_token};
use crate::models::{
    CommentListResponse, CommentRequest, CommentResponse, EngagementResponse, LikeRequest,
    LikeResponse,
};
use crate::proto::{auth, engagement};

pub async fn add_comment(
    State(state): State<SharedState>,
    Path(content_id): Path<String>,
    headers: HeaderMap,
    Json(payload): Json<CommentRequest>,
) -> Result<Json<CommentResponse>, ApiError> {
    let token = bearer_token(&headers)?;
    let user = validate_user(&state, &token).await?;

    let mut client = state.clients.engagement_client();
    let response = client
        .add_comment(engagement::AddCommentRequest {
            content_id,
            user_id: user.id,
            body: payload.body,
            created_at: 0,
        })
        .await
        .map_err(ApiError::from)?;

    let result = response.into_inner();
    let comment = result
        .comment
        .ok_or_else(|| ApiError::Upstream("missing comment payload".to_string()))?;
    Ok(Json(to_comment_response(comment)))
}

pub async fn list_comments(
    State(state): State<SharedState>,
    Path(content_id): Path<String>,
) -> Result<Json<CommentListResponse>, ApiError> {
    let mut client = state.clients.engagement_client();
    let response = client
        .list_comments(engagement::ListCommentsRequest { content_id })
        .await
        .map_err(ApiError::from)?;
    let result = response.into_inner();
    let items = result
        .comments
        .into_iter()
        .map(to_comment_response)
        .collect();
    Ok(Json(CommentListResponse { items }))
}

pub async fn set_like(
    State(state): State<SharedState>,
    Path(content_id): Path<String>,
    headers: HeaderMap,
    Json(payload): Json<LikeRequest>,
) -> Result<Json<LikeResponse>, ApiError> {
    let token = bearer_token(&headers)?;
    let user = validate_user(&state, &token).await?;

    let mut client = state.clients.engagement_client();
    let response = client
        .set_like(engagement::SetLikeRequest {
            content_id,
            user_id: user.id,
            liked: payload.liked,
        })
        .await
        .map_err(ApiError::from)?;
    let result = response.into_inner();
    Ok(Json(LikeResponse {
        likes: result.likes,
        liked: result.liked,
    }))
}

pub async fn get_engagement(
    State(state): State<SharedState>,
    Path(content_id): Path<String>,
    headers: HeaderMap,
) -> Result<Json<EngagementResponse>, ApiError> {
    let user_id = match optional_bearer_token(&headers)? {
        Some(token) => validate_user(&state, &token).await?.id,
        None => String::new(),
    };

    let mut client = state.clients.engagement_client();
    let response = client
        .get_engagement(engagement::GetEngagementRequest { content_id, user_id })
        .await
        .map_err(ApiError::from)?;
    let result = response.into_inner();
    Ok(Json(EngagementResponse {
        likes: result.likes,
        comments: result.comments,
        liked_by_user: result.liked_by_user,
    }))
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

fn optional_bearer_token(headers: &HeaderMap) -> Result<Option<String>, ApiError> {
    let header_value = match headers.get(header::AUTHORIZATION) {
        Some(value) => value,
        None => return Ok(None),
    };
    let header_value = header_value
        .to_str()
        .map_err(|_| ApiError::Unauthorized("invalid bearer token".to_string()))?;
    let token = header_value
        .strip_prefix("Bearer ")
        .ok_or_else(|| ApiError::Unauthorized("invalid bearer token".to_string()))?;
    if token.is_empty() {
        return Err(ApiError::Unauthorized("invalid bearer token".to_string()));
    }
    Ok(Some(token.to_string()))
}

fn to_comment_response(comment: engagement::Comment) -> CommentResponse {
    CommentResponse {
        id: comment.id,
        content_id: comment.content_id,
        user_id: comment.user_id,
        body: comment.body,
        created_at: comment.created_at,
    }
}
