//! Порты: контракты внешних сервисов, которые нужны сценариям. Реализации — в `library-infrastructure`.

use std::error::Error;

use async_trait::async_trait;
use bytes::Bytes;
use chrono::TimeDelta;
use library_domain::users::PasswordHash;
use uuid::Uuid;

use crate::auth::{Session, TokenClaims};

/// Ошибка внешнего сервиса (S3, SMTP, ...). Сценарий не знает, что внутри, — только что вызов не удался.
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct ExternalError(Box<dyn Error + Send + Sync>);

impl ExternalError {
    pub fn new(source: impl Into<Box<dyn Error + Send + Sync>>) -> Self {
        Self(source.into())
    }
}

pub type ExternalResult<T> = Result<T, ExternalError>;

/// Хэширование паролей (argon2).
#[async_trait]
pub trait PasswordHasher: Send + Sync {
    async fn hash(&self, password: String) -> ExternalResult<PasswordHash>;

    /// `false`, если пароль не подходит.
    async fn verify(&self, password: String, hash: PasswordHash) -> ExternalResult<bool>;
}

/// Подпись и проверка access-токенов (JWT).
pub trait TokenCodec: Send + Sync {
    fn encode(&self, claims: &TokenClaims) -> ExternalResult<String>;

    /// `None`, если токен не разбирается или подпись неверна. Срок действия не проверяет.
    fn decode(&self, token: &str) -> Option<TokenClaims>;
}

/// Хранилище сессий. Id сессии — он же refresh-токен.
#[async_trait]
pub trait SessionStore: Send + Sync {
    async fn save(&self, session: &Session, ttl: TimeDelta) -> ExternalResult<()>;

    async fn find(&self, session_id: Uuid) -> ExternalResult<Option<Session>>;

    async fn delete(&self, session_id: Uuid) -> ExternalResult<()>;
}

/// Хранилище файлов (S3).
#[async_trait]
pub trait FileStorage: Send + Sync {
    async fn put(&self, key: &str, content: Bytes, content_type: &str) -> ExternalResult<()>;
}

/// Почта. Письмо уходит в фоне: запрос его не ждёт, ошибки отправки логирует реализация.
pub trait Mailer: Send + Sync {
    fn send_verification_code(&self, email: &str, code: &str);
}

/// Ограничение частоты запросов (token bucket).
#[async_trait]
pub trait RateLimiter: Send + Sync {
    /// `true`, если запрос с этим ключом можно пропустить.
    async fn allow(&self, key: &str) -> ExternalResult<bool>;
}
