use async_trait::async_trait;

use crate::error::RepositoryResult;
use crate::materials::MaterialRepository;
use crate::submissions::SubmissionRepository;
use crate::users::UserRepository;

/// Единица работы: открывает транзакцию, внутри которой репозитории работают атомарно.
/// Реализация (sqlx) — в `library-infrastructure`.
#[async_trait]
pub trait UnitOfWork: Send + Sync {
    async fn begin(&self) -> RepositoryResult<Box<dyn Transaction>>;
}

/// Открытая транзакция. Без явного `commit` изменения откатываются при drop.
#[async_trait]
pub trait Transaction: Send + Sync {
    fn users(&self) -> Box<dyn UserRepository + '_>;

    fn materials(&self) -> Box<dyn MaterialRepository + '_>;

    fn submissions(&self) -> Box<dyn SubmissionRepository + '_>;

    async fn commit(self: Box<Self>) -> RepositoryResult<()>;

    async fn rollback(self: Box<Self>) -> RepositoryResult<()>;
}
