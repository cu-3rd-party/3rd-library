use uuid::Uuid;

use crate::users::UserDto;

/// Сессия пользователя; она же — результат проверки access-токена.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Session {
    pub user_id: Uuid,
    pub session_id: Uuid,
    /// Почта подтверждена на момент входа.
    pub verified: bool,
}

/// Содержимое access-токена.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TokenClaims {
    pub user_id: Uuid,
    pub session_id: Uuid,
    pub verified: bool,
    /// Unix-время истечения.
    pub expires_at: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenPairDto {
    pub access_token: String,
    /// Id сессии.
    pub refresh_token: String,
    /// Срок жизни сессии в секундах.
    pub expires_in: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthResultDto {
    pub user: UserDto,
    pub tokens: TokenPairDto,
}

#[derive(Debug, Clone)]
pub struct RegisterUser {
    pub name: String,
    pub email: String,
    pub password: String,
}
