use std::ops::{Deref, DerefMut};

use sqlx::pool::PoolConnection;
use sqlx::{PgConnection, PgPool, PgTransaction, Postgres};
use tokio::sync::{Mutex, MutexGuard};

use crate::metrics::observe_db_query;

/// Где работает репозиторий: на пуле (каждая запись — своя транзакция)
/// или внутри общей транзакции `UnitOfWork`.
#[derive(Clone)]
pub enum Db<'a> {
    Pool(PgPool),
    Transaction(&'a Mutex<PgTransaction<'static>>),
}

impl Db<'_> {
    /// Соединение для чтения.
    pub(crate) async fn conn(&self) -> sqlx::Result<Conn<'_>> {
        observe_db_query();
        Ok(match self {
            Db::Pool(pool) => Conn::Pool(pool.acquire().await?),
            Db::Transaction(txn) => Conn::Shared(txn.lock().await),
        })
    }

    /// Соединение для атомарной записи; изменения фиксирует [`Conn::commit`].
    pub(crate) async fn begin(&self) -> sqlx::Result<Conn<'_>> {
        observe_db_query();
        Ok(match self {
            Db::Pool(pool) => Conn::Own(pool.begin().await?),
            Db::Transaction(txn) => Conn::Shared(txn.lock().await),
        })
    }
}

pub(crate) enum Conn<'a> {
    Pool(PoolConnection<Postgres>),
    /// Своя транзакция метода репозитория.
    Own(PgTransaction<'static>),
    /// Общая транзакция `UnitOfWork`: фиксирует её владелец.
    Shared(MutexGuard<'a, PgTransaction<'static>>),
}

impl Conn<'_> {
    pub(crate) async fn commit(self) -> sqlx::Result<()> {
        match self {
            Conn::Own(txn) => txn.commit().await,
            Conn::Pool(_) | Conn::Shared(_) => Ok(()),
        }
    }
}

impl Deref for Conn<'_> {
    type Target = PgConnection;

    fn deref(&self) -> &PgConnection {
        match self {
            Conn::Pool(conn) => conn,
            Conn::Own(txn) => txn,
            Conn::Shared(txn) => txn,
        }
    }
}

impl DerefMut for Conn<'_> {
    fn deref_mut(&mut self) -> &mut PgConnection {
        match self {
            Conn::Pool(conn) => conn,
            Conn::Own(txn) => txn,
            Conn::Shared(txn) => txn,
        }
    }
}
