use crate::http::error::Error;
use axum::extract::FromRequestParts;

use crate::http::ApiContext;
use axum::http::HeaderValue;
use axum::http::header::AUTHORIZATION;
use axum::http::request::Parts;
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use redis::AsyncCommands;
use sha2::Sha384;
use time::OffsetDateTime;
use uuid::Uuid;

const DEFAULT_SESSION_LENGTH: time::Duration = time::Duration::weeks(2);

const SCHEME_PREFIX: &str = "Token ";

pub struct AuthUser {
    pub user_id: Uuid,
    pub session_id: Uuid,
}

/// Add this as a parameter to a handler function to optionally check if the user is logged in.
pub struct MaybeAuthUser(pub Option<AuthUser>);

#[derive(serde::Serialize, serde::Deserialize)]
struct AuthUserClaims {
    user_id: Uuid,
    sid: Uuid,
    exp: i64,
}

impl AuthUser {
    pub(in crate::http) async fn to_jwt(&self, ctx: &ApiContext) -> Result<String, Error> {
        let hmac = Hmac::<Sha384>::new_from_slice(ctx.config.jwt.hmac_key.as_bytes())
            .expect("HMAC-SHA-384 can accept any key length");

        let exp = (OffsetDateTime::now_utc() + DEFAULT_SESSION_LENGTH).unix_timestamp();

        let mut redis = ctx.redis.clone();
        let key = format!("session:{}", self.session_id);
        let ttl_seconds = DEFAULT_SESSION_LENGTH.whole_seconds().max(1) as u64;
        let _: () = redis
            .set_ex(key, self.user_id.to_string(), ttl_seconds)
            .await
            .map_err(|e| {
                log::error!("failed to set redis session: {}", e);
                Error::Anyhow(anyhow::anyhow!("redis error: {}", e))
            })?;

        AuthUserClaims {
            user_id: self.user_id,
            sid: self.session_id,
            exp,
        }
        .sign_with_key(&hmac)
        .map_err(|e| anyhow::anyhow!("HMAC signing failed: {}", e).into())
    }

    async fn from_authorization(
        ctx: &ApiContext,
        auth_header: &HeaderValue,
    ) -> Result<Self, Error> {
        let auth_header = auth_header.to_str().map_err(|_| {
            log::debug!("Authorization header is not UTF-8");
            Error::Unauthorized
        })?;

        if !auth_header.starts_with(SCHEME_PREFIX) {
            log::debug!(
                "Authorization header is using the wrong scheme: {:?}",
                auth_header
            );
            return Err(Error::Unauthorized);
        }

        let token = &auth_header[SCHEME_PREFIX.len()..];

        let jwt =
            jwt::Token::<jwt::Header, AuthUserClaims, _>::parse_unverified(token).map_err(|e| {
                log::debug!(
                    "failed to parse Authorization header {:?}: {}",
                    auth_header,
                    e
                );
                Error::Unauthorized
            })?;

        let hmac = Hmac::<Sha384>::new_from_slice(ctx.config.jwt.hmac_key.as_bytes())
            .expect("HMAC-SHA-384 can accept any key length");

        let jwt = jwt.verify_with_key(&hmac).map_err(|e| {
            log::debug!("JWT failed to verify: {}", e);
            Error::Unauthorized
        })?;

        let (_header, claims) = jwt.into();

        if claims.exp < OffsetDateTime::now_utc().unix_timestamp() {
            log::debug!("token expired");
            return Err(Error::Unauthorized);
        }

        let mut redis = ctx.redis.clone();
        let key = format!("session:{}", claims.sid);
        let session_user_id: Option<String> = redis.get(key).await.map_err(|e| {
            log::debug!("failed to read redis session: {}", e);
            Error::Unauthorized
        })?;

        let expected_user_id = claims.user_id.to_string();
        if session_user_id.as_deref() != Some(expected_user_id.as_str()) {
            log::debug!("session not found or mismatched");
            return Err(Error::Unauthorized);
        }

        Ok(Self {
            user_id: claims.user_id,
            session_id: claims.sid,
        })
    }
}

impl MaybeAuthUser {
    pub fn user_id(&self) -> Option<Uuid> {
        self.0.as_ref().map(|auth_user| auth_user.user_id)
    }
}

impl FromRequestParts<ApiContext> for AuthUser {
    type Rejection = Error;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &ApiContext,
    ) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get(AUTHORIZATION)
            .ok_or(Error::Unauthorized)?;

        Self::from_authorization(state, auth_header).await
    }
}

impl FromRequestParts<ApiContext> for MaybeAuthUser {
    type Rejection = Error;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &ApiContext,
    ) -> Result<Self, Self::Rejection> {
        let auth_user = if let Some(auth_header) = parts.headers.get(AUTHORIZATION) {
            Some(AuthUser::from_authorization(state, auth_header).await?)
        } else {
            None
        };

        Ok(Self(auth_user))
    }
}
