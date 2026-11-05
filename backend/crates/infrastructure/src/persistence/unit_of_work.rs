use async_trait::async_trait;
use library_domain::RepositoryError;
use library_domain::error::RepositoryResult;
use library_domain::materials::MaterialRepository;
use library_domain::submissions::SubmissionRepository;
use library_domain::unit_of_work::{Transaction, UnitOfWork};
use library_domain::users::UserRepository;
use sqlx::{PgPool, PgTransaction};
use tokio::sync::Mutex;

use super::Db;
use super::repositories::{SqlxMaterialRepository, SqlxSubmissionRepository, SqlxUserRepository};

pub struct SqlxUnitOfWork {
    db: PgPool,
}

impl SqlxUnitOfWork {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
}

#[async_trait]
impl UnitOfWork for SqlxUnitOfWork {
    async fn begin(&self) -> RepositoryResult<Box<dyn Transaction>> {
        let txn = self.db.begin().await.map_err(RepositoryError::new)?;
        Ok(Box::new(SqlxTransaction { txn: Mutex::new(txn) }))
    }
}

/// Репозитории транзакции по очереди берут её соединение (`Mutex`).
pub struct SqlxTransaction {
    txn: Mutex<PgTransaction<'static>>,
}

#[async_trait]
impl Transaction for SqlxTransaction {
    fn users(&self) -> Box<dyn UserRepository + '_> {
        Box::new(SqlxUserRepository::new(Db::Transaction(&self.txn)))
    }

    fn materials(&self) -> Box<dyn MaterialRepository + '_> {
        Box::new(SqlxMaterialRepository::new(Db::Transaction(&self.txn)))
    }

    fn submissions(&self) -> Box<dyn SubmissionRepository + '_> {
        Box::new(SqlxSubmissionRepository::new(Db::Transaction(&self.txn)))
    }

    async fn commit(self: Box<Self>) -> RepositoryResult<()> {
        self.txn.into_inner().commit().await.map_err(RepositoryError::new)
    }

    async fn rollback(self: Box<Self>) -> RepositoryResult<()> {
        self.txn.into_inner().rollback().await.map_err(RepositoryError::new)
    }
}
