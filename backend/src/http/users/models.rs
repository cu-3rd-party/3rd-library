use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

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

#[derive(Deserialize, ToSchema)]
#[allow(dead_code)] // keep it for now, because currently we don't have api for that
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

#[derive(Serialize, ToSchema)]
pub struct PaginatedUsersResponse {
    pub items: Vec<UserPublicProfile>,
    pub page: i64,
    pub limit: i64,
    pub total: i64,
}

#[derive(Serialize, ToSchema)]
pub struct UserWithMaterialsResponse {
    pub user: UserPublicProfile,
    pub materials: crate::http::materials::models::PaginatedMaterialsResponse,
}
