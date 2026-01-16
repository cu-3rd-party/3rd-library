use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Debug)]
pub enum ApiError {
    BadRequest(String),
    Unauthorized(String),
    NotFound(String),
    Conflict(String),
    Upstream(String),
}

#[derive(Serialize)]
struct ErrorBody {
    error: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            ApiError::Conflict(msg) => (StatusCode::CONFLICT, msg),
            ApiError::Upstream(msg) => (StatusCode::BAD_GATEWAY, msg),
        };
        (status, Json(ErrorBody { error: message })).into_response()
    }
}

impl From<tonic::Status> for ApiError {
    fn from(status: tonic::Status) -> Self {
        use tonic::Code;
        match status.code() {
            Code::InvalidArgument => ApiError::BadRequest(status.message().to_string()),
            Code::Unauthenticated => ApiError::Unauthorized(status.message().to_string()),
            Code::NotFound => ApiError::NotFound(status.message().to_string()),
            Code::AlreadyExists => ApiError::Conflict(status.message().to_string()),
            _ => ApiError::Upstream(status.message().to_string()),
        }
    }
}
