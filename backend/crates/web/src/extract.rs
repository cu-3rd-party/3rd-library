//! Обёртки над экстракторами axum: ошибки разбора запроса отдаются в формате ошибок API.

use axum::extract::{FromRequest, FromRequestParts, Request};
use axum::response::{IntoResponse, Response};

use crate::error::ApiError;

#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(ApiError))]
pub struct Json<T>(pub T);

impl<T: serde::Serialize> IntoResponse for Json<T> {
    fn into_response(self) -> Response {
        axum::Json(self.0).into_response()
    }
}

#[derive(FromRequestParts)]
#[from_request(via(axum::extract::Path), rejection(ApiError))]
pub struct Path<T>(pub T);

#[derive(FromRequestParts)]
#[from_request(via(axum::extract::Query), rejection(ApiError))]
pub struct Query<T>(pub T);

pub struct Multipart(pub axum::extract::Multipart);

impl<S: Send + Sync> FromRequest<S> for Multipart {
    type Rejection = ApiError;

    async fn from_request(request: Request, state: &S) -> Result<Self, Self::Rejection> {
        Ok(Self(axum::extract::Multipart::from_request(request, state).await?))
    }
}
