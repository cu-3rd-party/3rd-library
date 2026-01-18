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

#[derive(Debug, Deserialize)]
pub struct CommentRequest {
    pub body: String,
}

#[derive(Debug, Serialize)]
pub struct CommentResponse {
    pub id: String,
    pub content_id: String,
    pub user_id: String,
    pub body: String,
    pub created_at: i64,
}

#[derive(Debug, Serialize)]
pub struct CommentListResponse {
    pub items: Vec<CommentResponse>,
}

#[derive(Debug, Deserialize)]
pub struct LikeRequest {
    pub liked: bool,
}

#[derive(Debug, Serialize)]
pub struct LikeResponse {
    pub likes: i64,
    pub liked: bool,
}

#[derive(Debug, Serialize)]
pub struct EngagementResponse {
    pub likes: i64,
    pub comments: i64,
    pub liked_by_user: bool,
}
