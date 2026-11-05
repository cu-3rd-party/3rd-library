//! Доменный слой. Не зависит ни от sqlx, ни от axum: только сущности, value objects,
//! доменные ошибки и контракты репозиториев (реализации — в `library-infrastructure`).

pub mod common;
pub mod error;
pub mod materials;
pub mod submissions;
pub mod unit_of_work;
pub mod users;

pub use error::{DomainError, RepositoryError};
