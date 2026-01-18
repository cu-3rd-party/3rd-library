use std::time::{Duration, SystemTime, UNIX_EPOCH};

use bcrypt::{DEFAULT_COST, hash, verify};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: i64,
}

#[derive(Debug)]
pub enum AuthError {
    InvalidInput,
    InvalidCredentials,
    EmailTaken,
    UserNotFound,
    TokenInvalid,
    Internal(String),
}

#[derive(Clone)]
pub struct AuthService {
    pool: PgPool,
    token_manager: TokenManager,
}

impl AuthService {
    pub fn new(pool: PgPool, secret: String, ttl: Duration) -> Self {
        Self {
            pool,
            token_manager: TokenManager::new(secret, ttl),
        }
    }

    pub async fn register(&self, name: String, email: String, password: String) -> Result<(User, String), AuthError> {
        let name = name.trim().to_string();
        let email = email.trim().to_lowercase();
        if name.is_empty() || email.is_empty() || password.is_empty() || !email.contains('@') {
            return Err(AuthError::InvalidInput);
        }

        let password_hash = hash(password, DEFAULT_COST)
            .map_err(|err| AuthError::Internal(err.to_string()))?;
        let created_at = now_timestamp();
        let user = User {
            id: Uuid::new_v4().simple().to_string(),
            name,
            email: email.clone(),
            password_hash,
            created_at,
        };

        let result = sqlx::query(
            r#"
            INSERT INTO users (id, name, email, password_hash, created_at)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (email) DO NOTHING
            "#,
        )
        .bind(&user.id)
        .bind(&user.name)
        .bind(&user.email)
        .bind(&user.password_hash)
        .bind(user.created_at)
        .execute(&self.pool)
        .await
        .map_err(map_sqlx_error)?;

        if result.rows_affected() == 0 {
            return Err(AuthError::EmailTaken);
        }

        let token = self.token_manager.issue(&user)?;
        Ok((user, token))
    }

    pub async fn login(&self, email: String, password: String) -> Result<(User, String), AuthError> {
        let email = email.trim().to_lowercase();
        if email.is_empty() || password.is_empty() {
            return Err(AuthError::InvalidCredentials);
        }

        let row = sqlx::query_as::<_, UserRow>(
            r#"
            SELECT id, name, email, password_hash, created_at
            FROM users
            WHERE email = $1
            "#,
        )
        .bind(&email)
        .fetch_optional(&self.pool)
        .await
        .map_err(map_sqlx_error)?
        .ok_or(AuthError::InvalidCredentials)?;

        let user = row.into_user();
        if !verify(password, &user.password_hash).map_err(|err| AuthError::Internal(err.to_string()))? {
            return Err(AuthError::InvalidCredentials);
        }

        let token = self.token_manager.issue(&user)?;
        Ok((user, token))
    }

    pub async fn validate(&self, token: &str) -> Result<User, AuthError> {
        let token = token.trim();
        if token.is_empty() {
            return Err(AuthError::TokenInvalid);
        }
        let user_id = self.token_manager.parse(token)?;

        let row = sqlx::query_as::<_, UserRow>(
            r#"
            SELECT id, name, email, password_hash, created_at
            FROM users
            WHERE id = $1
            "#,
        )
        .bind(&user_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(map_sqlx_error)?
        .ok_or(AuthError::UserNotFound)?;
        Ok(row.into_user())
    }
}

#[derive(Debug, FromRow)]
struct UserRow {
    id: String,
    name: String,
    email: String,
    password_hash: String,
    created_at: i64,
}

impl UserRow {
    fn into_user(self) -> User {
        User {
            id: self.id,
            name: self.name,
            email: self.email,
            password_hash: self.password_hash,
            created_at: self.created_at,
        }
    }
}

#[derive(Clone)]
struct TokenManager {
    encoding: EncodingKey,
    decoding: DecodingKey,
    ttl: Duration,
}

impl TokenManager {
    fn new(secret: String, ttl: Duration) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret.as_bytes()),
            decoding: DecodingKey::from_secret(secret.as_bytes()),
            ttl,
        }
    }

    fn issue(&self, user: &User) -> Result<String, AuthError> {
        let now = now_timestamp() as usize;
        let exp = now.saturating_add(self.ttl.as_secs() as usize);
        let claims = Claims {
            sub: user.id.clone(),
            exp,
            iat: now,
        };
        encode(&Header::default(), &claims, &self.encoding)
            .map_err(|err| AuthError::Internal(err.to_string()))
    }

    fn parse(&self, token: &str) -> Result<String, AuthError> {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = true;
        let data = decode::<Claims>(token, &self.decoding, &validation)
            .map_err(|_| AuthError::TokenInvalid)?;
        Ok(data.claims.sub)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
    iat: usize,
}

fn map_sqlx_error(err: sqlx::Error) -> AuthError {
    AuthError::Internal(err.to_string())
}

fn now_timestamp() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_user() -> User {
        User {
            id: "user-123".to_string(),
            name: "Ada".to_string(),
            email: "ada@example.com".to_string(),
            password_hash: "hash".to_string(),
            created_at: 1_700_000_000,
        }
    }

    #[test]
    fn token_round_trip() {
        let manager = TokenManager::new("secret".to_string(), Duration::from_secs(60));
        let user = sample_user();
        let token = manager.issue(&user).expect("token issued");
        let parsed = manager.parse(&token).expect("token parsed");
        assert_eq!(parsed, user.id);
    }

    #[test]
    fn invalid_token_rejected() {
        let manager = TokenManager::new("secret".to_string(), Duration::from_secs(60));
        let err = manager.parse("not-a-jwt").expect_err("invalid token rejected");
        assert!(matches!(err, AuthError::TokenInvalid));
    }
}
