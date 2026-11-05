use std::collections::BTreeMap;

use library_domain::{DomainError, RepositoryError};

use crate::ports::ExternalError;

/// Ошибки по полям: `{"email": ["must be a valid email address"]}`.
pub type ValidationErrors = BTreeMap<String, Vec<String>>;

/// Ошибка сценария. Web-слой переводит её в ответ с нужным статусом.
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("One or more validation failures have occurred.")]
    Validation(ValidationErrors),

    #[error("{entity} {id} was not found.")]
    NotFound { entity: &'static str, id: String },

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Forbidden")]
    Forbidden,

    #[error("{0}")]
    Conflict(String),

    #[error("Too many requests")]
    TooManyRequests,

    #[error(transparent)]
    Repository(#[from] RepositoryError),

    #[error(transparent)]
    External(#[from] ExternalError),
}

impl AppError {
    pub fn not_found(entity: &'static str, id: impl ToString) -> Self {
        Self::NotFound {
            entity,
            id: id.to_string(),
        }
    }

    pub fn validation(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Validation(BTreeMap::from([(field.into(), vec![message.into()])]))
    }
}

impl From<DomainError> for AppError {
    fn from(error: DomainError) -> Self {
        match error {
            DomainError::Validation { field, message } => Self::validation(field, message),
            DomainError::Conflict(message) => Self::Conflict(message),
            DomainError::InvalidState(message) => Self::validation("status", message),
        }
    }
}

pub type AppResult<T> = Result<T, AppError>;
