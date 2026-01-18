use axum::{
    Json,
    extract::{Path, State},
    http::HeaderMap,
};

use crate::auth::{AuthError, User};
use crate::engagement::{Comment, EngagementError};
use crate::errors::ApiError;
use crate::handlers::{SharedState, bearer_token};
use crate::models::{
    CommentListResponse, CommentRequest, CommentResponse, EngagementResponse, LikeRequest, LikeResponse,
};

pub async fn add_comment(
    State(state): State<SharedState>,
    Path(content_id): Path<String>,
    headers: HeaderMap,
    Json(payload): Json<CommentRequest>,
) -> Result<Json<CommentResponse>, ApiError> {
    let token = bearer_token(&headers).ok_or_else(|| ApiError::Unauthorized("missing bearer token".to_string()))?;
    let user = validate_user(&state, &token).await?;

    let comment = state
        .engagement
        .add_comment(content_id, user.id, payload.body, 0)
        .await
        .map_err(map_engagement_error)?;

    Ok(Json(to_comment_response(comment)))
}

pub async fn list_comments(
    State(state): State<SharedState>,
    Path(content_id): Path<String>,
) -> Result<Json<CommentListResponse>, ApiError> {
    let comments = state
        .engagement
        .list_comments(content_id)
        .await
        .map_err(map_engagement_error)?;
    let items = comments.into_iter().map(to_comment_response).collect();
    Ok(Json(CommentListResponse { items }))
}

pub async fn set_like(
    State(state): State<SharedState>,
    Path(content_id): Path<String>,
    headers: HeaderMap,
    Json(payload): Json<LikeRequest>,
) -> Result<Json<LikeResponse>, ApiError> {
    let token = bearer_token(&headers).ok_or_else(|| ApiError::Unauthorized("missing bearer token".to_string()))?;
    let user = validate_user(&state, &token).await?;

    let likes = state
        .engagement
        .set_like(content_id, user.id, payload.liked)
        .await
        .map_err(map_engagement_error)?;

    Ok(Json(LikeResponse {
        likes,
        liked: payload.liked,
    }))
}

pub async fn get_engagement(
    State(state): State<SharedState>,
    Path(content_id): Path<String>,
    headers: HeaderMap,
) -> Result<Json<EngagementResponse>, ApiError> {
    let user_id = match bearer_token(&headers) {
        Some(token) => Some(validate_user(&state, &token).await?.id),
        None => None,
    };

    let summary = state
        .engagement
        .summary(content_id, user_id)
        .await
        .map_err(map_engagement_error)?;

    Ok(Json(EngagementResponse {
        likes: summary.likes,
        comments: summary.comments,
        liked_by_user: summary.liked_by_user,
    }))
}

async fn validate_user(state: &SharedState, token: &str) -> Result<User, ApiError> {
    state.auth.validate(token).await.map_err(map_auth_error)
}

fn to_comment_response(comment: Comment) -> CommentResponse {
    CommentResponse {
        id: comment.id,
        content_id: comment.content_id,
        user_id: comment.user_id,
        body: comment.body,
        created_at: comment.created_at,
    }
}

fn map_engagement_error(err: EngagementError) -> ApiError {
    match err {
        EngagementError::InvalidInput(msg) => ApiError::BadRequest(msg),
        EngagementError::Db(msg) => ApiError::Internal(msg),
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
