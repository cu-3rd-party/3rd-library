use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: String,
    pub name: String,
    pub email: String,
    pub created_at: i64,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserResponse,
}

#[derive(Debug, Serialize)]
pub struct ContentMetadata {
    pub id: String,
    pub owner_id: String,
    pub title: String,
    pub description: String,
    pub filename: String,
    pub size_bytes: i64,
    pub created_at: i64,
}

#[derive(Debug, Serialize)]
pub struct ContentListResponse {
    pub items: Vec<ContentMetadata>,
    pub next_page_token: String,
}

#[derive(Debug, Serialize)]
pub struct StatsResponse {
    pub views: i64,
    pub downloads: i64,
    pub unique_users: i64,
    pub last_interaction_at: i64,
}
