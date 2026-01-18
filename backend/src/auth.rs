use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use bcrypt::{DEFAULT_COST, hash, verify};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
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
    store: std::sync::Arc<Mutex<AuthStore>>,
    token_manager: TokenManager,
}

impl AuthService {
    pub fn new(secret: String, ttl: Duration) -> Self {
        Self {
            store: std::sync::Arc::new(Mutex::new(AuthStore::default())),
            token_manager: TokenManager::new(secret, ttl),
        }
    }

    pub async fn register(&self, name: String, email: String, password: String) -> Result<(User, String), AuthError> {
        let name = name.trim().to_string();
        let email = email.trim().to_lowercase();
        if name.is_empty() || email.is_empty() || password.is_empty() || !email.contains('@') {
            return Err(AuthError::InvalidInput);
        }

        let mut store = self.store.lock().await;
        if store.by_email.contains_key(&email) {
            return Err(AuthError::EmailTaken);
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

        store.by_email.insert(email, user.id.clone());
        store.by_id.insert(user.id.clone(), user.clone());

        let token = self.token_manager.issue(&user)?;
        Ok((user, token))
    }

    pub async fn login(&self, email: String, password: String) -> Result<(User, String), AuthError> {
        let email = email.trim().to_lowercase();
        if email.is_empty() || password.is_empty() {
            return Err(AuthError::InvalidCredentials);
        }

        let store = self.store.lock().await;
        let user_id = store
            .by_email
            .get(&email)
            .ok_or(AuthError::InvalidCredentials)?;
        let user = store
            .by_id
            .get(user_id)
            .ok_or(AuthError::InvalidCredentials)?
            .clone();
        drop(store);

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
        let store = self.store.lock().await;
        let user = store.by_id.get(&user_id).cloned().ok_or(AuthError::UserNotFound)?;
        Ok(user)
    }
}

#[derive(Default)]
struct AuthStore {
    by_id: HashMap<String, User>,
    by_email: HashMap<String, String>,
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

fn now_timestamp() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}
