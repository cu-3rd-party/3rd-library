mod like;
pub mod repositories;
mod rows;
mod store;
mod writer;

use std::time::Duration;

use library_migration::MIGRATOR;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

pub use self::store::{AUTH_SYNC_INTERVAL, Store};

/// Пул: полосы группового коммита, чтение ролей, загрузка при старте. Чтения данных в БД не ходят.
pub async fn connect(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(16)
        .acquire_timeout(Duration::from_secs(5))
        .connect(database_url)
        .await
}

/// Применяет миграции. Ошибка не мешает старту: в прод-БД бывают версии,
/// применённые другими сборками бэкенда (sqlx тогда отказывается мигрировать) — как и раньше, только предупреждаем.
pub async fn migrate(db: &PgPool) {
    match MIGRATOR.run(db).await {
        Ok(()) => tracing::info!("database migrations are up to date"),
        Err(error) => tracing::warn!(%error, "skipping migrations"),
    }
}
