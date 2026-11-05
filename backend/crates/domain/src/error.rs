use std::error::Error;

/// Нарушение инварианта доменной модели.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum DomainError {
    /// Значение не прошло проверку.
    #[error("{field}: {message}")]
    Validation { field: &'static str, message: String },

    /// Операция уже выполнена или противоречит существующим данным.
    #[error("{0}")]
    Conflict(String),

    /// Операция недопустима в текущем состоянии сущности.
    #[error("{0}")]
    InvalidState(&'static str),
}

impl DomainError {
    pub fn validation(field: &'static str, message: impl Into<String>) -> Self {
        Self::Validation {
            field,
            message: message.into(),
        }
    }
}

/// Ошибка хранилища. Домен не знает, что внутри (sqlx, ...), — только что операция не удалась.
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct RepositoryError(Box<dyn Error + Send + Sync>);

impl RepositoryError {
    pub fn new(source: impl Error + Send + Sync + 'static) -> Self {
        Self(Box::new(source))
    }
}

pub type RepositoryResult<T> = Result<T, RepositoryError>;
