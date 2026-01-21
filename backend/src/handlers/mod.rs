use axum::http::HeaderMap;
use std::sync::Arc;

use crate::auth::AuthService;
use crate::content::ContentService;
use crate::engagement::EngagementService;
use crate::statistics::StatisticsService;

pub mod auth;
pub mod content;
pub mod docs;
pub mod engagement;

#[derive(Clone)]
pub struct AppState {
    pub auth: AuthService,
    pub content: ContentService,
    pub engagement: EngagementService,
    pub statistics: StatisticsService,
    pub max_upload_bytes: usize,
    pub docs_html: String,
}

pub type SharedState = Arc<AppState>;

pub fn bearer_token(headers: &HeaderMap) -> Option<String> {
    let header = headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .unwrap_or("");
    let token = header.strip_prefix("Bearer ").unwrap_or("");
    if token.is_empty() {
        None
    } else {
        Some(token.to_string())
    }
}

pub fn anonymous_id(headers: &HeaderMap) -> String {
    headers
        .get("x-anonymous-id")
        .and_then(|value| value.to_str().ok())
        .unwrap_or("anonymous")
        .to_string()
}
