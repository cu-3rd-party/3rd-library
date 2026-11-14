//! Миграции схемы — SQL-файлы в `migrations/` (sqlx). Применяются при старте web, руками — через CLI этого крейта.
//!
//! Уже применённые файлы не менять: sqlx сверяет их контрольные суммы с записанными в `_sqlx_migrations`.

pub use sqlx::migrate::Migrator;

/// Без advisory lock: sqlx берёт сессионный `pg_advisory_lock`, а через transaction-пулер PgBouncer
/// unlock уходит на другое серверное соединение, и lock висит, пока пулер не закроет первое.
/// Следующий старт тогда ждёт его без таймаута. Мигрирует одна реплика (Recreate), гонок нет.
pub static MIGRATOR: Migrator = {
    let mut migrator = sqlx::migrate!("./migrations");
    migrator.locking = false;
    migrator
};
