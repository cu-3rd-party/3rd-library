//! Слой приложения: сценарии использования поверх доменных абстракций.
//! Сервисы получают репозитории и порты (`ports`) как `Arc<dyn Trait>` и ничего не знают о sqlx, S3, SMTP и HTTP.

mod access;
pub mod auth;
pub mod common;
pub mod error;
pub mod materials;
pub mod moderation;
pub mod ports;
pub mod submissions;
pub mod users;

#[cfg(test)]
mod testing;

pub use error::{AppError, AppResult};
