mod login;
mod logout;
mod refresh;
mod register;
mod resend_verification;
mod verify_email;

pub mod models;

use crate::http::ApiContext;
use axum::Router;
use axum::routing::post;
#[allow(unused_imports)]
use crate::http::materials::create::__path_create_submission;
#[allow(unused_imports)]
use crate::http::materials::get::__path_get_material;
#[allow(unused_imports)]
use crate::http::materials::get_single::__path_get_submission_by_id;
#[allow(unused_imports)]
use crate::http::materials::list::{__path_list_materials, __path_list_submissions};
#[allow(unused_imports)]
use crate::http::materials::update::__path_update_submission;
#[allow(unused_imports)]
use crate::http::moderation::decision::__path_moderation_decision;
#[allow(unused_imports)]
use crate::http::moderation::list::__path_list_moderation_submissions;
#[allow(unused_imports)]
use crate::http::users::get::{__path_get_current_user, __path_get_user_by_id, __path_get_users};
#[allow(unused_imports)]
use crate::http::users::update::__path_update_user_profile;
use login::__path_login_user;
use logout::__path_logout;
use refresh::__path_refresh_token;
use register::__path_register_user;
use resend_verification::__path_resend_verification_code;
use utoipa::{Modify, OpenApi};
use utoipa::openapi::security::{Http, HttpAuthScheme, SecurityScheme};
use verify_email::__path_verify_email;

pub use login::login_user;
pub use logout::logout;
pub use refresh::refresh_token;
pub use register::register_user;
pub use resend_verification::resend_verification_code;
pub use verify_email::verify_email;

use crate::http::HealthCheck;
use crate::http::error::{ApiError, FieldError};
use crate::http::materials::models::{
    CreateSubmissionRequest, ListMaterialsQuery, ListSubmissionsQuery, Material, MaterialDetails,
    MaterialFile, PaginatedMaterialsResponse, PaginatedSubmissionsResponse, Submission,
    UpdateSubmissionRequest,
};
use crate::http::moderation::models::{
    ModerationDecisionRequest, ModerationQuery, PaginatedModerationResponse,
    SubmissionStatusCounters,
};
use crate::http::users::models::{
    ListUsersQuery, PaginatedUsersResponse, UpdateUserRequest, UserPublicProfile,
    UserWithMaterialsResponse,
};
use models::{
    AuthResponse, LoginRequest, RefreshTokenRequest, RegisterRequest, RegisterResponse,
    ResendVerificationCodeRequest, TokenPair, VerifyEmailRequest, WebUser,
};

pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
            );
        }
    }
}

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::http::health_check,
        register_user,
        verify_email,
        resend_verification_code,
        login_user,
        refresh_token,
        logout,
        crate::http::users::get::get_current_user,
        crate::http::users::get::get_users,
        crate::http::users::get::get_user_by_id,
        crate::http::users::update::update_user_profile,
        crate::http::materials::list::list_materials,
        crate::http::materials::get::get_material,
        crate::http::materials::list::list_submissions,
        crate::http::materials::create::create_submission,
        crate::http::materials::get_single::get_submission_by_id,
        crate::http::materials::update::update_submission,
        crate::http::moderation::list::list_moderation_submissions,
        crate::http::moderation::decision::moderation_decision
    ),
    components(
        schemas(
            HealthCheck,
            ApiError,
            FieldError,
            RegisterRequest,
            VerifyEmailRequest,
            ResendVerificationCodeRequest,
            LoginRequest,
            RefreshTokenRequest,
            WebUser,
            TokenPair,
            AuthResponse,
            RegisterResponse,
            ListUsersQuery,
            UpdateUserRequest,
            UserPublicProfile,
            PaginatedUsersResponse,
            UserWithMaterialsResponse,
            Material,
            MaterialFile,
            MaterialDetails,
            PaginatedMaterialsResponse,
            ListMaterialsQuery,
            ListSubmissionsQuery,
            Submission,
            PaginatedSubmissionsResponse,
            CreateSubmissionRequest,
            UpdateSubmissionRequest,
            ModerationDecisionRequest,
            ModerationQuery,
            SubmissionStatusCounters,
            PaginatedModerationResponse
        )
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "auth", description = "Authentication endpoints"),
        (name = "users", description = "User profile and discovery endpoints"),
        (name = "materials", description = "Material catalogue and submission endpoints"),
        (name = "moderation", description = "Moderator-only review endpoints"),
        (name = "system", description = "Service health and system endpoints")
    ),
    info(
        title = "3rd Library Backend API",
        description = "OpenAPI documentation for the 3rd-library backend"
    )
)]
pub struct ApiDoc;

pub fn router() -> Router<ApiContext> {
    Router::new()
        .route("/api/auth/register", post(register_user))
        .route("/api/auth/verify-email", post(verify_email))
        .route(
            "/api/auth/resend-verification-code",
            post(resend_verification_code),
        )
        .route("/api/auth/login", post(login_user))
        .route("/api/auth/refresh", post(refresh_token))
        .route("/api/auth/logout", post(logout))
}
