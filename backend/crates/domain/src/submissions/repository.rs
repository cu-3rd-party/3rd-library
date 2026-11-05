use async_trait::async_trait;

use super::{Submission, SubmissionId, SubmissionStatus};
use crate::common::{Page, PageRequest};
use crate::error::RepositoryResult;
use crate::users::UserId;

/// Число заявок по статусам (для очереди модерации).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct StatusCounters {
    pub all: i64,
    pub draft: i64,
    pub pending_review: i64,
    pub rejected: i64,
    pub approved: i64,
}

/// Репозиторий агрегата (заявка загружается вместе с файлами). Реализация (sqlx) — в `library-infrastructure`.
#[async_trait]
pub trait SubmissionRepository: Send + Sync {
    async fn find_by_id(&self, id: SubmissionId) -> RepositoryResult<Option<Submission>>;

    /// Заявки автора, новые сверху; `status: None` — все.
    async fn list_by_author(
        &self,
        author: UserId,
        status: Option<SubmissionStatus>,
        page: PageRequest,
    ) -> RepositoryResult<Page<Submission>>;

    /// Очередь модерации, старые сверху; `status: None` — все.
    async fn list_for_moderation(
        &self,
        status: Option<SubmissionStatus>,
        page: PageRequest,
    ) -> RepositoryResult<Page<Submission>>;

    async fn count_by_status(&self) -> RepositoryResult<StatusCounters>;

    /// Заявка, её новые файлы и связи с ними.
    async fn insert(&self, submission: &Submission) -> RepositoryResult<()>;

    /// `false`, если записи с таким id нет. Новые файлы сохраняются, связи с убранными удаляются.
    async fn update(&self, submission: &Submission) -> RepositoryResult<bool>;
}
