use axum::http::HeaderMap;
use std::sync::Arc;

use crate::clients::Clients;
use crate::errors::ApiError;

pub mod auth;
pub mod content;
pub mod engagement;

#[derive(Clone)]
pub struct AppState {
    pub clients: Clients,
    pub max_upload_bytes: usize,
}

impl AppState {
    pub fn new(clients: Clients, max_upload_bytes: usize) -> Self {
        Self {
            clients,
            max_upload_bytes,
        }
    }
}

pub fn bearer_token(headers: &HeaderMap) -> Result<String, ApiError> {
    let header = headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .unwrap_or("");
    let token = header.strip_prefix("Bearer ").unwrap_or("");
    if token.is_empty() {
        return Err(ApiError::Unauthorized("missing bearer token".to_string()));
    }
    Ok(token.to_string())
}

pub fn anonymous_id(headers: &HeaderMap) -> String {
    headers
        .get("x-anonymous-id")
        .and_then(|value| value.to_str().ok())
        .unwrap_or("anonymous")
        .to_string()
}

pub type SharedState = Arc<AppState>;
