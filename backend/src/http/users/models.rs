use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct WebUser {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub bio: String,
    pub is_email_verified: bool,
    pub can_submit_materials: bool,
    pub roles: Vec<String>,
}

#[derive(Deserialize)]
pub struct UpdateUserRequest {
    pub name: Option<String>,
    pub bio: Option<String>,
}
