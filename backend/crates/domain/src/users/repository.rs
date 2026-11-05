use async_trait::async_trait;

use super::{User, UserId};
use crate::common::{Page, PageRequest};
use crate::error::RepositoryResult;

/// Публичная карточка автора для списка пользователей.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorSummary {
    pub id: UserId,
    pub name: String,
    pub bio: String,
    pub is_email_verified: bool,
    pub materials_count: i64,
}

/// Репозиторий агрегата. Реализация (sqlx) — в `library-infrastructure`.
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: UserId) -> RepositoryResult<Option<User>>;

    async fn find_by_email(&self, email: &str) -> RepositoryResult<Option<User>>;

    /// `false`, если email уже занят.
    async fn insert(&self, user: &User) -> RepositoryResult<bool>;

    /// `false`, если записи с таким id нет.
    async fn update(&self, user: &User) -> RepositoryResult<bool>;

    /// Авторы — пользователи хотя бы с одним опубликованным материалом, по имени;
    /// `search` — подстрока имени без учёта регистра.
    async fn list_authors(&self, search: Option<String>, page: PageRequest) -> RepositoryResult<Page<AuthorSummary>>;
}
