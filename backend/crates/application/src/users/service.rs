use std::sync::Arc;

use library_domain::common::PageRequest;
use library_domain::materials::MaterialRepository;
use library_domain::users::{User, UserId, UserRepository};
use uuid::Uuid;

use super::{AuthorDto, AuthorWithMaterialsDto, UpdateProfile, UserDto};
use crate::common::PageDto;
use crate::materials::MaterialDto;
use crate::{AppError, AppResult};

const ENTITY: &str = "User";

#[derive(Clone)]
pub struct UserService {
    users: Arc<dyn UserRepository>,
    materials: Arc<dyn MaterialRepository>,
}

impl UserService {
    pub fn new(users: Arc<dyn UserRepository>, materials: Arc<dyn MaterialRepository>) -> Self {
        Self { users, materials }
    }

    pub async fn current(&self, user_id: Uuid) -> AppResult<UserDto> {
        Ok(UserDto::from(&self.find(user_id).await?))
    }

    pub async fn update_profile(&self, user_id: Uuid, request: UpdateProfile) -> AppResult<UserDto> {
        let mut user = self.find(user_id).await?;
        user.update_profile(request.name, request.bio);

        if !self.users.update(&user).await? {
            return Err(AppError::not_found(ENTITY, user_id));
        }
        Ok(UserDto::from(&user))
    }

    /// Авторы опубликованных материалов.
    pub async fn list_authors(&self, search: Option<String>, page: PageRequest) -> AppResult<PageDto<AuthorDto>> {
        let authors = self.users.list_authors(search, page).await?;
        Ok(PageDto::new(authors, page, AuthorDto::from))
    }

    /// Профиль автора и страница его опубликованных материалов.
    pub async fn get_author(&self, user_id: Uuid, page: PageRequest) -> AppResult<AuthorWithMaterialsDto> {
        let user = self.find(user_id).await?;
        let materials = self.materials.list_published_by_author(user.id(), page).await?;

        Ok(AuthorWithMaterialsDto {
            author: AuthorDto {
                id: user_id,
                name: user.name().to_owned(),
                bio: user.bio().to_owned(),
                is_email_verified: user.is_email_verified(),
                materials_count: materials.total,
            },
            materials: PageDto::new(materials, page, MaterialDto::from),
        })
    }

    async fn find(&self, user_id: Uuid) -> AppResult<User> {
        self.users
            .find_by_id(UserId::from(user_id))
            .await?
            .ok_or_else(|| AppError::not_found(ENTITY, user_id))
    }
}
