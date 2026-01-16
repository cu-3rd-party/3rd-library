use axum::{Json, extract::State};

use crate::errors::ApiError;
use crate::handlers::SharedState;
use crate::models::{AuthResponse, LoginRequest, RegisterRequest, UserResponse};
use crate::proto::auth;

pub async fn register(
    State(state): State<SharedState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, ApiError> {
    let mut client = state.clients.auth_client();
    let response = client
        .register(auth::RegisterRequest {
            name: payload.name,
            email: payload.email,
            password: payload.password,
        })
        .await
        .map_err(ApiError::from)?;

    let result = response.into_inner();
    Ok(Json(AuthResponse {
        token: result.token,
        user: to_user_response(result.user)?,
    }))
}

pub async fn login(
    State(state): State<SharedState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, ApiError> {
    let mut client = state.clients.auth_client();
    let response = client
        .login(auth::LoginRequest {
            email: payload.email,
            password: payload.password,
        })
        .await
        .map_err(ApiError::from)?;

    let result = response.into_inner();
    Ok(Json(AuthResponse {
        token: result.token,
        user: to_user_response(result.user)?,
    }))
}

fn to_user_response(user: Option<auth::User>) -> Result<UserResponse, ApiError> {
    let user = user.ok_or_else(|| ApiError::Upstream("missing user payload".to_string()))?;
    Ok(UserResponse {
        id: user.id,
        name: user.name,
        email: user.email,
        created_at: user.created_at,
    })
}
