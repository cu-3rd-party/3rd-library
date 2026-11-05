use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordVerifier};
use async_trait::async_trait;
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use library_application::auth::TokenClaims;
use library_application::ports::{ExternalError, ExternalResult, PasswordHasher, TokenCodec};
use library_domain::users::PasswordHash;
use serde::{Deserialize, Serialize};
use sha2::Sha384;
use uuid::Uuid;

/// Argon2 (параметры по умолчанию), считается в blocking-пуле.
pub struct Argon2PasswordHasher;

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
        tokio::task::spawn_blocking(move || {
            let hash = argon2::PasswordHash::new(hash.as_str())
                .map_err(|error| ExternalError::new(format!("invalid password hash: {error}")))?;
            match Argon2::default().verify_password(password.as_bytes(), &hash) {
                Ok(()) => Ok(true),
                Err(argon2::password_hash::Error::Password) => Ok(false),
                Err(error) => Err(ExternalError::new(format!("failed to verify password: {error}"))),
            }
        })
        .await
        .map_err(ExternalError::new)?
    }
}

/// JWT (HS384). Поля токена не менять: иначе выданные прежними версиями токены станут недействительны.
pub struct JwtTokenCodec {
    key: Hmac<Sha384>,
}

impl JwtTokenCodec {
    pub fn new(hmac_key: &str) -> Self {
        Self {
            key: Hmac::new_from_slice(hmac_key.as_bytes()).expect("HMAC-SHA-384 accepts any key length"),
        }
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
        let claims: Claims = token
            .verify_with_key(&self.key)
            .inspect_err(|error| tracing::debug!(%error, "JWT failed to verify"))
            .ok()?;
        Some(TokenClaims {
            user_id: claims.user_id,
            session_id: claims.sid,
            verified: claims.verified,
            expires_at: claims.exp,
        })
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
        let hash = Argon2PasswordHasher.hash("secret".into()).await.unwrap();

        assert!(
            Argon2PasswordHasher
                .verify("secret".into(), hash.clone())
                .await
                .unwrap()
        );
        assert!(!Argon2PasswordHasher.verify("wrong".into(), hash).await.unwrap());
    }
}
