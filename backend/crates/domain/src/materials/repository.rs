use async_trait::async_trait;
use chrono::{DateTime, Utc};

use super::{Material, MaterialFile, MaterialId, MaterialInfo};
use crate::common::{Page, PageRequest};
use crate::error::RepositoryResult;
use crate::users::UserId;

/// Материал для каталога: с именем автора, без файлов.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaterialSummary {
    pub id: MaterialId,
    pub author_id: UserId,
    pub author_name: String,
    pub info: MaterialInfo,
    pub published_at: Option<DateTime<Utc>>,
    pub created: DateTime<Utc>,
}

/// Репозиторий агрегата. Реализация (sqlx) — в `library-infrastructure`.
#[async_trait]
pub trait MaterialRepository: Send + Sync {
    /// Опубликованные материалы, новые сверху; `search` — подстрока названия без учёта регистра.
    async fn list_published(
        &self,
        search: Option<String>,
        page: PageRequest,
    ) -> RepositoryResult<Page<MaterialSummary>>;

    /// Опубликованные материалы автора, новые сверху.
    async fn list_published_by_author(
        &self,
        author: UserId,
        page: PageRequest,
    ) -> RepositoryResult<Page<MaterialSummary>>;

    async fn find_by_id(&self, id: MaterialId) -> RepositoryResult<Option<MaterialSummary>>;

    async fn files(&self, id: MaterialId) -> RepositoryResult<Vec<MaterialFile>>;

    /// Материал и связи с его файлами (сами файлы уже сохранены вместе с заявкой).
    async fn insert(&self, material: &Material) -> RepositoryResult<()>;
}
