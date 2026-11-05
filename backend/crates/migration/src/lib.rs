//! Миграции схемы — SQL-файлы в `migrations/` (sqlx). Применяются при старте web, руками — через CLI этого крейта.
//!
//! Уже применённые файлы не менять: sqlx сверяет их контрольные суммы с записанными в `_sqlx_migrations`.

pub use sqlx::migrate::Migrator;

pub static MIGRATOR: Migrator = sqlx::migrate!("./migrations");
