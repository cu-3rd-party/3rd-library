use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct VerifyEmailRequest {
    pub email: String,
    pub code: String,
}

#[derive(Deserialize)]
pub struct ResendVerificationCodeRequest {
    pub email: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

#[derive(Serialize)]
pub struct WebUser {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub bio: String,
    pub is_email_verified: bool,
    pub can_submit_materials: bool,
    pub roles: Vec<String>,
}

#[derive(Serialize)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub user: WebUser,
    pub tokens: TokenPair,
}

#[derive(Serialize)]
pub struct RegisterResponse {
    pub user: WebUser,
    pub verification_required: bool,
    pub verification_channel: Option<String>,
}
