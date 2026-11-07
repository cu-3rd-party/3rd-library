use std::collections::HashMap;
use std::sync::{Arc, Mutex, PoisonError};

use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordVerifier};
use async_trait::async_trait;
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use library_application::auth::TokenClaims;
use library_application::ports::{ExternalError, ExternalResult, PasswordHasher, TokenCodec};
use library_domain::users::PasswordHash;
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Sha384};
use uuid::Uuid;

/// Сколько записей держат кеши проверок; при переполнении кеш просто очищается.
const CACHE_CAPACITY: usize = 1 << 17;

/// Argon2 (параметры по умолчанию), считается в blocking-пуле.
///
/// Проверка пароля — чистая функция от `(хеш, пароль)` (соль внутри хеша), поэтому её результат
/// кешируется: argon2 считается один раз на пару. Ключ кеша — HMAC-SHA-256 пары на случайном ключе
/// процесса, сами пароли в памяти не хранятся.
pub struct Argon2PasswordHasher {
    key: Hmac<Sha256>,
    verified: Arc<Mutex<HashMap<[u8; 32], bool>>>,
}

impl Argon2PasswordHasher {
    pub fn new() -> Self {
        let secret: [u8; 32] = rand::random();
        Self {
            key: Hmac::new_from_slice(&secret).expect("HMAC accepts any key length"),
            verified: Arc::default(),
        }
    }

    fn cache_key(&self, password: &str, hash: &PasswordHash) -> [u8; 32] {
        let mut mac = self.key.clone();
        mac.update(&(hash.as_str().len() as u64).to_le_bytes());
        mac.update(hash.as_str().as_bytes());
        mac.update(password.as_bytes());
        mac.finalize().into_bytes().into()
    }
}

impl Default for Argon2PasswordHasher {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl PasswordHasher for Argon2PasswordHasher {
    async fn hash(&self, password: String) -> ExternalResult<PasswordHash> {
        tokio::task::spawn_blocking(move || {
            let salt = SaltString::generate(rand::thread_rng());
            argon2::PasswordHash::generate(Argon2::default(), password, &salt)
                .map(|hash| PasswordHash::new(hash.to_string()))
                .map_err(|error| ExternalError::new(format!("failed to hash password: {error}")))
        })
        .await
        .map_err(ExternalError::new)?
    }

    async fn verify(&self, password: String, hash: PasswordHash) -> ExternalResult<bool> {
        let key = self.cache_key(&password, &hash);
        if let Some(valid) = self.verified.lock().unwrap_or_else(PoisonError::into_inner).get(&key) {
            return Ok(*valid);
        }
        let valid = tokio::task::spawn_blocking(move || {
            let hash = argon2::PasswordHash::new(hash.as_str())
                .map_err(|error| ExternalError::new(format!("invalid password hash: {error}")))?;
            match Argon2::default().verify_password(password.as_bytes(), &hash) {
                Ok(()) => Ok(true),
                Err(argon2::password_hash::Error::Password) => Ok(false),
                Err(error) => Err(ExternalError::new(format!("failed to verify password: {error}"))),
            }
        })
        .await
        .map_err(ExternalError::new)??;

        let mut verified = self.verified.lock().unwrap_or_else(PoisonError::into_inner);
        if verified.len() >= CACHE_CAPACITY {
            verified.clear();
        }
        verified.insert(key, valid);
        Ok(valid)
    }
}

/// JWT (HS384). Поля токена не менять: иначе выданные прежними версиями токены станут недействительны.
///
/// Разбор токена (base64, подпись, JSON) — чистая функция строки, поэтому успешно проверенные токены
/// кешируются целиком (ключ — сама строка токена); срок действия проверяет вызывающий на каждом запросе.
pub struct JwtTokenCodec {
    key: Hmac<Sha384>,
    decoded: Mutex<HashMap<Box<str>, TokenClaims>>,
}

impl JwtTokenCodec {
    pub fn new(hmac_key: &str) -> Self {
        Self {
            key: Hmac::new_from_slice(hmac_key.as_bytes()).expect("HMAC-SHA-384 accepts any key length"),
            decoded: Mutex::default(),
        }
    }

    fn cache(&self) -> std::sync::MutexGuard<'_, HashMap<Box<str>, TokenClaims>> {
        self.decoded.lock().unwrap_or_else(PoisonError::into_inner)
    }
}

#[derive(Serialize, Deserialize)]
struct Claims {
    user_id: Uuid,
    sid: Uuid,
    verified: bool,
    exp: i64,
}

impl TokenCodec for JwtTokenCodec {
    fn encode(&self, claims: &TokenClaims) -> ExternalResult<String> {
        Claims {
            user_id: claims.user_id,
            sid: claims.session_id,
            verified: claims.verified,
            exp: claims.expires_at,
        }
        .sign_with_key(&self.key)
        .map_err(ExternalError::new)
    }

    fn decode(&self, token: &str) -> Option<TokenClaims> {
        if let Some(claims) = self.cache().get(token) {
            return Some(*claims);
        }
        let claims: Claims = token
            .verify_with_key(&self.key)
            .inspect_err(|error| tracing::debug!(%error, "JWT failed to verify"))
            .ok()?;
        let claims = TokenClaims {
            user_id: claims.user_id,
            session_id: claims.sid,
            verified: claims.verified,
            expires_at: claims.exp,
        };
        let mut cache = self.cache();
        if cache.len() >= CACHE_CAPACITY {
            cache.clear();
        }
        cache.insert(token.into(), claims);
        Some(claims)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn claims() -> TokenClaims {
        TokenClaims {
            user_id: Uuid::new_v4(),
            session_id: Uuid::new_v4(),
            verified: true,
            expires_at: 1_900_000_000,
        }
    }

    #[test]
    fn jwt_roundtrip() {
        let codec = JwtTokenCodec::new("key");
        let claims = claims();

        let token = codec.encode(&claims).unwrap();

        assert_eq!(codec.decode(&token), Some(claims));
    }

    #[test]
    fn jwt_with_other_key_is_rejected() {
        let token = JwtTokenCodec::new("key").encode(&claims()).unwrap();

        assert_eq!(JwtTokenCodec::new("other").decode(&token), None);
        assert_eq!(JwtTokenCodec::new("key").decode("not-a-token"), None);
    }

    #[tokio::test]
    async fn argon2_roundtrip() {
        let hasher = Argon2PasswordHasher::new();
        let hash = hasher.hash("secret".into()).await.unwrap();

        for _ in 0..2 {
            assert!(hasher.verify("secret".into(), hash.clone()).await.unwrap());
            assert!(!hasher.verify("wrong".into(), hash.clone()).await.unwrap());
        }
    }
}
