use axum::extract::State;
use library_application::common::{PageDto, PageRequest};
use library_application::users::{AuthorDto, UpdateProfile, UserDto};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;
use uuid::Uuid;

use crate::AppState;
use crate::auth::AuthUser;
use crate::error::{ApiResult, ErrorBody};
use crate::extract::{Json, Path, Query};
use crate::routes::materials::PaginatedMaterialsResponse;

const TAG: &str = "users";

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(get_current_user, update_user_profile))
        .routes(routes!(get_users))
        .routes(routes!(get_user_by_id))
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct WebUser {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub bio: String,
    pub is_email_verified: bool,
    pub can_submit_materials: bool,
    pub roles: Vec<String>,
}

impl From<UserDto> for WebUser {
    fn from(dto: UserDto) -> Self {
        Self {
            id: dto.id,
            name: dto.name,
            email: dto.email,
            bio: dto.bio,
            is_email_verified: dto.is_email_verified,
            can_submit_materials: dto.can_submit_materials,
            roles: dto.roles,
        }
    }
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateUserRequest {
    pub name: Option<String>,
    pub bio: Option<String>,
}

#[derive(Deserialize, IntoParams, ToSchema)]
pub struct ListUsersQuery {
    pub search: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Serialize, ToSchema)]
pub struct UserPublicProfile {
    pub id: Uuid,
    pub name: String,
    pub bio: String,
    pub is_email_verified: bool,
    pub materials_count: i64,
}

impl From<AuthorDto> for UserPublicProfile {
    fn from(dto: AuthorDto) -> Self {
        Self {
            id: dto.id,
            name: dto.name,
            bio: dto.bio,
            is_email_verified: dto.is_email_verified,
            materials_count: dto.materials_count,
        }
    }
}

#[derive(Serialize, ToSchema)]
pub struct PaginatedUsersResponse {
    pub items: Vec<UserPublicProfile>,
    pub page: i64,
    pub limit: i64,
    pub total: i64,
}

impl From<PageDto<AuthorDto>> for PaginatedUsersResponse {
    fn from(page: PageDto<AuthorDto>) -> Self {
        Self {
            items: page.items.into_iter().map(Into::into).collect(),
            page: page.page,
            limit: page.limit,
            total: page.total,
        }
    }
}

#[derive(Serialize, ToSchema)]
pub struct UserWithMaterialsResponse {
    pub user: UserPublicProfile,
    pub materials: PaginatedMaterialsResponse,
}

#[utoipa::path(get, path = "/api/users/me", tag = TAG,
    responses(
        (status = 200, description = "Current user profile", body = WebUser),
        (status = 401, description = "Authentication required", body = ErrorBody),
    ),
    security(("bearer_auth" = [])))]
async fn get_current_user(user: AuthUser, State(state): State<AppState>) -> ApiResult<Json<WebUser>> {
    Ok(Json(state.users.current(user.user_id()).await?.into()))
}

#[utoipa::path(patch, path = "/api/users/me", tag = TAG, request_body = UpdateUserRequest,
    responses(
        (status = 200, description = "User profile updated", body = WebUser),
        (status = 401, description = "Authentication required", body = ErrorBody),
    ),
    security(("bearer_auth" = [])))]
async fn update_user_profile(
    user: AuthUser,
    State(state): State<AppState>,
    Json(request): Json<UpdateUserRequest>,
) -> ApiResult<Json<WebUser>> {
    let profile = UpdateProfile {
        name: request.name,
        bio: request.bio,
    };
    Ok(Json(state.users.update_profile(user.user_id(), profile).await?.into()))
}

/// Авторы опубликованных материалов
#[utoipa::path(get, path = "/api/users", tag = TAG, params(ListUsersQuery),
    responses((status = 200, description = "Public users list", body = PaginatedUsersResponse)))]
async fn get_users(
    State(state): State<AppState>,
    Query(query): Query<ListUsersQuery>,
) -> ApiResult<Json<PaginatedUsersResponse>> {
    let page = PageRequest::new(query.page, query.limit);
    Ok(Json(state.users.list_authors(query.search, page).await?.into()))
}

#[utoipa::path(get, path = "/api/users/{userId}", tag = TAG,
    params(("userId" = Uuid, Path, description = "User id"), ListUsersQuery),
    responses(
        (status = 200, description = "Public user profile with materials", body = UserWithMaterialsResponse),
        (status = 404, description = "User not found", body = ErrorBody),
    ))]
async fn get_user_by_id(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    Query(query): Query<ListUsersQuery>,
) -> ApiResult<Json<UserWithMaterialsResponse>> {
    let author = state
        .users
        .get_author(user_id, PageRequest::new(query.page, query.limit))
        .await?;
    Ok(Json(UserWithMaterialsResponse {
        user: author.author.into(),
        materials: author.materials.into(),
    }))
}
