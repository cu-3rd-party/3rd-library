use axum::extract::multipart::{MultipartError, MultipartRejection};
use axum::extract::rejection::{JsonRejection, PathRejection, QueryRejection};
use axum::http::header::WWW_AUTHENTICATE;
use axum::http::{HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use library_application::AppError;
use serde::Serialize;
use utoipa::ToSchema;

/// Тело ошибки API (формат прежнего бэкенда, его ждёт фронтенд): `{"code", "message", "details"}`.
#[derive(Debug, Serialize, ToSchema)]
#[schema(as = ApiError)]
pub struct ErrorBody {
    pub code: &'static str,
    pub message: String,
    /// Ошибки по полям (для `bad_request` из-за валидации).
    pub details: Option<Vec<FieldError>>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct FieldError {
    pub field: String,
    pub message: String,
}

/// Ошибка обработчика: всё, что может пойти не так, превращается в [`ErrorBody`].
#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error(transparent)]
    App(#[from] AppError),

    /// Запрос не разобран (тело, путь, query, multipart).
    #[error("{message}")]
    Rejection { status: StatusCode, message: String },
}

impl ApiError {
    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::Rejection {
            status: StatusCode::BAD_REQUEST,
            message: message.into(),
        }
    }
}

macro_rules! from_rejection {
    ($($rejection:ty),*) => {$(
        impl From<$rejection> for ApiError {
            fn from(rejection: $rejection) -> Self {
                Self::Rejection {
                    status: rejection.status(),
                    message: rejection.body_text(),
                }
            }
        }
    )*};
}

from_rejection!(
    JsonRejection,
    PathRejection,
    QueryRejection,
    MultipartRejection,
    MultipartError
);

pub type ApiResult<T> = Result<T, ApiError>;

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, code, message, details) = match self {
            ApiError::App(AppError::Validation(errors)) => {
                let details = errors
                    .into_iter()
                    .flat_map(|(field, messages)| {
                        messages.into_iter().map(move |message| FieldError {
                            field: field.clone(),
                            message,
                        })
                    })
                    .collect();
                (
                    StatusCode::BAD_REQUEST,
                    "bad_request",
                    "bad request".into(),
                    Some(details),
                )
            }
            ApiError::App(AppError::NotFound { .. }) => (
                StatusCode::NOT_FOUND,
                "not_found",
                "request path not found".into(),
                None,
            ),
            ApiError::App(AppError::Unauthorized) => {
                let body = ErrorBody {
                    code: "unauthorized",
                    message: "authentication required".into(),
                    details: None,
                };
                return (
                    StatusCode::UNAUTHORIZED,
                    [(WWW_AUTHENTICATE, HeaderValue::from_static("Bearer"))],
                    axum::Json(body),
                )
                    .into_response();
            }
            ApiError::App(AppError::Forbidden) => (
                StatusCode::FORBIDDEN,
                "forbidden",
                "user may not perform that action".into(),
                None,
            ),
            ApiError::App(AppError::Conflict(message)) => (StatusCode::CONFLICT, "conflict", message, None),
            ApiError::App(AppError::TooManyRequests) => (
                StatusCode::TOO_MANY_REQUESTS,
                "too_many_requests",
                "too many requests".into(),
                None,
            ),
            ApiError::App(AppError::Repository(error)) => {
                tracing::error!(%error, "unhandled repository error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal_error",
                    "an error occurred with the database".into(),
                    None,
                )
            }
            ApiError::App(AppError::External(error)) => {
                tracing::error!(%error, "unhandled external service error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal_error",
                    "an internal server error occurred".into(),
                    None,
                )
            }
            ApiError::Rejection { status, message } => (status, rejection_code(status), message, None),
        };

        (status, axum::Json(ErrorBody { code, message, details })).into_response()
    }
}

fn rejection_code(status: StatusCode) -> &'static str {
    match status {
        StatusCode::PAYLOAD_TOO_LARGE => "payload_too_large",
        StatusCode::UNSUPPORTED_MEDIA_TYPE => "unsupported_media_type",
        StatusCode::UNPROCESSABLE_ENTITY => "validation_error",
        status if status.is_server_error() => "internal_error",
        _ => "bad_request",
    }
}
