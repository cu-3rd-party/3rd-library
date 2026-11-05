use library_domain::users::{AuthorSummary, User};
use uuid::Uuid;

use crate::common::PageDto;
use crate::materials::MaterialDto;

/// Пользователь глазами его самого.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserDto {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub bio: String,
    pub is_email_verified: bool,
    pub can_submit_materials: bool,
    pub roles: Vec<String>,
}

impl From<&User> for UserDto {
    fn from(user: &User) -> Self {
        Self {
            id: user.id().into_inner(),
            name: user.name().to_owned(),
            email: user.email().to_string(),
            bio: user.bio().to_owned(),
            is_email_verified: user.is_email_verified(),
            can_submit_materials: user.can_submit_materials(),
            roles: user.roles().to_vec(),
        }
    }
}

/// Публичный профиль автора.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorDto {
    pub id: Uuid,
    pub name: String,
    pub bio: String,
    pub is_email_verified: bool,
    pub materials_count: i64,
}

impl From<AuthorSummary> for AuthorDto {
    fn from(author: AuthorSummary) -> Self {
        Self {
            id: author.id.into_inner(),
            name: author.name,
            bio: author.bio,
            is_email_verified: author.is_email_verified,
            materials_count: author.materials_count,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorWithMaterialsDto {
    pub author: AuthorDto,
    pub materials: PageDto<MaterialDto>,
}

#[derive(Debug, Clone, Default)]
pub struct UpdateProfile {
    pub name: Option<String>,
    pub bio: Option<String>,
}
