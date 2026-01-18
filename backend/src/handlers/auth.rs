use axum::{Json, extract::State};

use crate::auth::{AuthError, User};
use crate::errors::ApiError;
use crate::handlers::SharedState;
use crate::models::{AuthResponse, LoginRequest, RegisterRequest, UserResponse};

pub async fn register(
    State(state): State<SharedState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, ApiError> {
    let (user, token) = state
        .auth
        .register(payload.name, payload.email, payload.password)
        .await
        .map_err(map_auth_error)?;
    Ok(Json(AuthResponse {
        token,
        user: to_user_response(&user),
    }))
}

pub async fn login(
    State(state): State<SharedState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, ApiError> {
    let (user, token) = state
        .auth
        .login(payload.email, payload.password)
        .await
        .map_err(map_auth_error)?;
    Ok(Json(AuthResponse {
        token,
        user: to_user_response(&user),
    }))
}

fn to_user_response(user: &User) -> UserResponse {
    UserResponse {
        id: user.id.clone(),
        name: user.name.clone(),
        email: user.email.clone(),
        created_at: user.created_at,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_auth_error_maps_variants() {
        assert!(matches!(
            map_auth_error(AuthError::InvalidInput),
            ApiError::BadRequest(_)
        ));
        assert!(matches!(
            map_auth_error(AuthError::InvalidCredentials),
            ApiError::Unauthorized(_)
        ));
        assert!(matches!(
            map_auth_error(AuthError::EmailTaken),
            ApiError::Conflict(_)
        ));
        assert!(matches!(
            map_auth_error(AuthError::UserNotFound),
            ApiError::Unauthorized(_)
        ));
        assert!(matches!(
            map_auth_error(AuthError::TokenInvalid),
            ApiError::Unauthorized(_)
        ));
        assert!(matches!(
            map_auth_error(AuthError::Internal("boom".to_string())),
            ApiError::Internal(_)
        ));
    }

    #[test]
    fn to_user_response_copies_fields() {
        let user = User {
            id: "user-1".to_string(),
            name: "Ada".to_string(),
            email: "ada@example.com".to_string(),
            password_hash: "hash".to_string(),
            created_at: 42,
        };
        let response = to_user_response(&user);
        assert_eq!(response.id, "user-1");
        assert_eq!(response.name, "Ada");
        assert_eq!(response.email, "ada@example.com");
        assert_eq!(response.created_at, 42);
    }
}
