//! Экстракторы аутентификации: заголовок `Authorization: Token <jwt>` (или `Bearer <jwt>`, как в Swagger UI).

use axum::extract::FromRequestParts;
use axum::http::header::AUTHORIZATION;
use axum::http::request::Parts;
use library_application::AppError;
use library_application::auth::Session;
use uuid::Uuid;

use crate::AppState;
use crate::error::ApiError;

/// Аутентифицированный пользователь (почта может быть не подтверждена).
pub struct AuthUser(pub Session);

/// Пользователь с подтверждённой почтой.
pub struct VerifiedUser(pub Session);

/// Пользователь, если заголовок есть; неверный токен — всё равно 401.
pub struct MaybeAuthUser(pub Option<Session>);

impl AuthUser {
    pub fn user_id(&self) -> Uuid {
        self.0.user_id
    }
}

impl VerifiedUser {
    pub fn user_id(&self) -> Uuid {
        self.0.user_id
    }
}

async fn authenticate(parts: &Parts, state: &AppState) -> Result<Option<Session>, ApiError> {
    let Some(header) = parts.headers.get(AUTHORIZATION) else {
        return Ok(None);
    };
    let token = header
        .to_str()
        .ok()
        .and_then(|value| value.strip_prefix("Token ").or_else(|| value.strip_prefix("Bearer ")))
        .ok_or(AppError::Unauthorized)?;
    Ok(Some(state.auth.authenticate(token).await?))
}

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let session = authenticate(parts, state).await?.ok_or(AppError::Unauthorized)?;
        Ok(Self(session))
    }
}

impl FromRequestParts<AppState> for VerifiedUser {
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let AuthUser(session) = AuthUser::from_request_parts(parts, state).await?;
        if !session.verified {
            return Err(AppError::Unauthorized.into());
        }
        Ok(Self(session))
    }
}

impl FromRequestParts<AppState> for MaybeAuthUser {
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        Ok(Self(authenticate(parts, state).await?))
    }
}
