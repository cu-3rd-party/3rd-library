use axum::extract::State;
use library_application::auth::{AuthResultDto, RegisterUser, TokenPairDto};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::AppState;
use crate::auth::AuthUser;
use crate::error::{ApiResult, ErrorBody};
use crate::extract::Json;
use crate::routes::users::WebUser;

const TAG: &str = "auth";

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(register_user))
        .routes(routes!(verify_email))
        .routes(routes!(resend_verification_code))
        .routes(routes!(login_user))
        .routes(routes!(refresh_token))
        .routes(routes!(logout))
}

#[derive(Deserialize, ToSchema)]
pub struct RegisterRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, ToSchema)]
pub struct VerifyEmailRequest {
    pub email: String,
    pub code: String,
}

#[derive(Deserialize, ToSchema)]
pub struct ResendVerificationCodeRequest {
    pub email: String,
}

#[derive(Deserialize, ToSchema)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, ToSchema)]
pub struct RefreshTokenRequest {
    #[serde(rename = "refreshToken")]
    pub refresh_token: String,
}

#[derive(Serialize, ToSchema)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u64,
}

impl From<TokenPairDto> for TokenPair {
    fn from(dto: TokenPairDto) -> Self {
        Self {
            access_token: dto.access_token,
            refresh_token: dto.refresh_token,
            expires_in: dto.expires_in,
        }
    }
}

#[derive(Serialize, ToSchema)]
pub struct AuthResponse {
    pub user: WebUser,
    pub tokens: TokenPair,
}

impl From<AuthResultDto> for AuthResponse {
    fn from(dto: AuthResultDto) -> Self {
        Self {
            user: dto.user.into(),
            tokens: dto.tokens.into(),
        }
    }
}

#[derive(Serialize, ToSchema)]
pub struct RegisterResponse {
    pub user: WebUser,
    pub verification_required: bool,
    pub verification_channel: Option<String>,
}

#[utoipa::path(post, path = "/api/auth/register", tag = TAG, request_body = RegisterRequest,
    responses(
        (status = 200, description = "User registered successfully", body = RegisterResponse),
        (status = 400, description = "Invalid request", body = ErrorBody),
        (status = 409, description = "Email already exists", body = ErrorBody),
    ))]
async fn register_user(
    State(state): State<AppState>,
    Json(request): Json<RegisterRequest>,
) -> ApiResult<Json<RegisterResponse>> {
    let user = state
        .auth
        .register(RegisterUser {
            name: request.name,
            email: request.email,
            password: request.password,
        })
        .await?;
    Ok(Json(RegisterResponse {
        user: user.into(),
        verification_required: true,
        verification_channel: Some("email_code".into()),
    }))
}

#[utoipa::path(post, path = "/api/auth/verify-email", tag = TAG, request_body = VerifyEmailRequest,
    responses(
        (status = 200, description = "Email verified successfully", body = AuthResponse),
        (status = 400, description = "Invalid or expired verification code", body = ErrorBody),
        (status = 404, description = "User not found", body = ErrorBody),
        (status = 409, description = "Email already verified", body = ErrorBody),
    ))]
async fn verify_email(
    State(state): State<AppState>,
    Json(request): Json<VerifyEmailRequest>,
) -> ApiResult<Json<AuthResponse>> {
    let result = state.auth.verify_email(&request.email, &request.code).await?;
    Ok(Json(result.into()))
}

#[utoipa::path(post, path = "/api/auth/resend-verification-code", tag = TAG,
    request_body = ResendVerificationCodeRequest,
    responses(
        (status = 200, description = "Verification code resent"),
        (status = 404, description = "User not found or already verified", body = ErrorBody),
        (status = 429, description = "Verification code requested too recently", body = ErrorBody),
    ))]
async fn resend_verification_code(
    State(state): State<AppState>,
    Json(request): Json<ResendVerificationCodeRequest>,
) -> ApiResult<Json<()>> {
    state.auth.resend_verification_code(&request.email).await?;
    Ok(Json(()))
}

#[utoipa::path(post, path = "/api/auth/login", tag = TAG, request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = AuthResponse),
        (status = 401, description = "Invalid credentials", body = ErrorBody),
        (status = 403, description = "Email not verified", body = ErrorBody),
    ))]
async fn login_user(State(state): State<AppState>, Json(request): Json<LoginRequest>) -> ApiResult<Json<AuthResponse>> {
    let result = state.auth.login(&request.email, request.password).await?;
    Ok(Json(result.into()))
}

#[utoipa::path(post, path = "/api/auth/refresh", tag = TAG, request_body = RefreshTokenRequest,
    responses(
        (status = 200, description = "Tokens refreshed", body = TokenPair),
        (status = 401, description = "Invalid refresh token", body = ErrorBody),
    ))]
async fn refresh_token(
    State(state): State<AppState>,
    Json(request): Json<RefreshTokenRequest>,
) -> ApiResult<Json<TokenPair>> {
    Ok(Json(state.auth.refresh(&request.refresh_token).await?.into()))
}

#[utoipa::path(post, path = "/api/auth/logout", tag = TAG,
    responses(
        (status = 200, description = "Logged out successfully"),
        (status = 401, description = "Authentication required", body = ErrorBody),
    ),
    security(("bearer_auth" = [])))]
async fn logout(user: AuthUser, State(state): State<AppState>) -> ApiResult<()> {
    state.auth.logout(user.0.session_id).await?;
    Ok(())
}
