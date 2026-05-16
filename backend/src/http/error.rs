use axum::Json;
use axum::http::header::WWW_AUTHENTICATE;
use axum::http::{HeaderMap, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use utoipa::ToSchema;
use sqlx::error::DatabaseError;
use std::borrow::Cow;
use std::collections::HashMap;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("authentication required")]
    Unauthorized,

    #[error("user may not perform that action")]
    Forbidden,

    #[error("request path not found")]
    NotFound,

    #[error("bad request")]
    BadRequest,

    #[error("too many requests")]
    TooManyRequests,

    #[error("conflict")]
    Conflict(String),

    #[error("error in the request body")]
    UnprocessableEntity {
        errors: HashMap<Cow<'static, str>, Vec<Cow<'static, str>>>,
    },

    #[error("an error occurred with the database")]
    Sqlx(#[from] sqlx::Error),

    #[error("an internal server error occurred")]
    Anyhow(#[from] anyhow::Error),
}

#[derive(Serialize, ToSchema)]
pub struct ApiError {
    code: String,
    message: String,
    details: Option<Vec<FieldError>>,
}

#[derive(Serialize, ToSchema)]
pub struct FieldError {
    field: String,
    message: String,
}

impl Error {
    pub fn api_error(code: impl Into<String>, message: impl Into<String>) -> Self {
        let mut error_map = HashMap::new();
        error_map
            .entry(Cow::Owned(code.into()))
            .or_insert_with(Vec::new)
            .push(Cow::Owned(message.into()));
        Self::UnprocessableEntity { errors: error_map }
    }

    pub fn bad_request(_code: impl Into<String>, _message: impl Into<String>) -> Self {
        Self::BadRequest
    }

    pub fn unauthorized(_code: impl Into<String>, _message: impl Into<String>) -> Self {
        Self::Unauthorized
    }

    pub fn forbidden(_code: impl Into<String>, _message: impl Into<String>) -> Self {
        Self::Forbidden
    }

    pub fn not_found(_code: impl Into<String>, _message: impl Into<String>) -> Self {
        Self::NotFound
    }

    pub fn conflict(_code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Conflict(message.into())
    }

    pub fn too_many_requests() -> Self {
        Self::TooManyRequests
    }
}

impl Error {
    pub fn unprocessable_entity<K, V>(errors: impl IntoIterator<Item = (K, V)>) -> Self
    where
        K: Into<Cow<'static, str>>,
        V: Into<Cow<'static, str>>,
    {
        let mut error_map = HashMap::new();

        for (key, val) in errors {
            error_map
                .entry(key.into())
                .or_insert_with(Vec::new)
                .push(val.into());
        }

        Self::UnprocessableEntity { errors: error_map }
    }

    #[allow(dead_code)]
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::Forbidden => StatusCode::FORBIDDEN,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::BadRequest => StatusCode::BAD_REQUEST,
            Self::TooManyRequests => StatusCode::TOO_MANY_REQUESTS,
            Self::Conflict(_) => StatusCode::CONFLICT,
            Self::UnprocessableEntity { .. } => StatusCode::UNPROCESSABLE_ENTITY,
            Self::Sqlx(_) | Self::Anyhow(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, code, message): (StatusCode, &str, String) = match &self {
            Self::Unauthorized => (StatusCode::UNAUTHORIZED, "unauthorized", self.to_string()),
            Self::Forbidden => (StatusCode::FORBIDDEN, "forbidden", self.to_string()),
            Self::NotFound => (StatusCode::NOT_FOUND, "not_found", self.to_string()),
            Self::BadRequest => (StatusCode::BAD_REQUEST, "bad_request", self.to_string()),
            Self::TooManyRequests => (
                StatusCode::TOO_MANY_REQUESTS,
                "too_many_requests",
                self.to_string(),
            ),
            Self::Conflict(msg) => (StatusCode::CONFLICT, "conflict", msg.clone()),
            Self::UnprocessableEntity { .. } => (
                StatusCode::UNPROCESSABLE_ENTITY,
                "validation_error",
                self.to_string(),
            ),
            Self::Sqlx(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error",
                "an error occurred with the database".to_string(),
            ),
            Self::Anyhow(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error",
                "an internal server error occurred".to_string(),
            ),
        };

        let api_error = ApiError {
            code: code.to_string(),
            message,
            details: None,
        };

        match self {
            Self::Unauthorized => {
                return (
                    status,
                    [(WWW_AUTHENTICATE, HeaderValue::from_static("Bearer"))]
                        .into_iter()
                        .collect::<HeaderMap>(),
                    Json(api_error),
                )
                    .into_response();
            }
            Self::Sqlx(ref e) => {
                log::error!("SQLx error: {:?}", e);
            }
            Self::Anyhow(ref e) => {
                log::error!("Generic error: {:?}", e);
            }
            _ => (),
        }

        (status, Json(api_error)).into_response()
    }
}

/// A little helper trait for more easily converting database constraint errors into API errors.
///
/// Something like this would ideally live in a `sqlx-axum` crate if it made sense to author one,
/// however its definition is tied pretty intimately to the `Error` type, which is itself
/// tied directly to application semantics.
///
/// To actually make this work in a generic context would make it quite a bit more complex,
/// as you'd need an intermediate error type to represent either a mapped or an unmapped error,
/// and even then it's not clear how to handle `?` in the unmapped case without more boilerplate.
pub trait ResultExt<T> {
    /// If `self` contains a SQLx database constraint error with the given name,
    /// transform the error.
    ///
    /// Otherwise, the result is passed through unchanged.
    fn on_constraint(
        self,
        name: &str,
        f: impl FnOnce(Box<dyn DatabaseError>) -> Error,
    ) -> Result<T, Error>;
}

impl<T, E> ResultExt<T> for Result<T, E>
where
    E: Into<Error>,
{
    fn on_constraint(
        self,
        name: &str,
        map_err: impl FnOnce(Box<dyn DatabaseError>) -> Error,
    ) -> Result<T, Error> {
        self.map_err(|e| match e.into() {
            Error::Sqlx(sqlx::Error::Database(dbe)) if dbe.constraint() == Some(name) => {
                map_err(dbe)
            }
            e => e,
        })
    }
}
