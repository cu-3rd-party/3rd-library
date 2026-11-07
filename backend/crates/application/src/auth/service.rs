use std::sync::Arc;

use chrono::{TimeDelta, Utc};
use library_domain::common::now;
use library_domain::users::{Email, User, UserRepository, VerificationCode};
use uuid::Uuid;

use super::{AuthResultDto, RegisterUser, Session, TokenClaims, TokenPairDto};
use crate::locks::lock;
use crate::ports::{Mailer, PasswordHasher, SessionStore, TokenCodec};
use crate::users::UserDto;
use crate::{AppError, AppResult};

const ENTITY: &str = "User";

/// Срок жизни сессии и access-токена.
pub const SESSION_TTL: TimeDelta = TimeDelta::weeks(2);

/// Регистрация, подтверждение почты, вход и сессии.
#[derive(Clone)]
pub struct AuthService {
    users: Arc<dyn UserRepository>,
    hasher: Arc<dyn PasswordHasher>,
    tokens: Arc<dyn TokenCodec>,
    sessions: Arc<dyn SessionStore>,
    mailer: Arc<dyn Mailer>,
}

impl AuthService {
    pub fn new(
        users: Arc<dyn UserRepository>,
        hasher: Arc<dyn PasswordHasher>,
        tokens: Arc<dyn TokenCodec>,
        sessions: Arc<dyn SessionStore>,
        mailer: Arc<dyn Mailer>,
    ) -> Self {
        Self {
            users,
            hasher,
            tokens,
            sessions,
            mailer,
        }
    }

    /// Новый пользователь получает код подтверждения на почту; войти можно после подтверждения.
    pub async fn register(&self, request: RegisterUser) -> AppResult<UserDto> {
        let email = Email::parse(&request.email)?;
        let password_hash = self.hasher.hash(request.password).await?;
        let code = VerificationCode::generate();
        let user = User::register(email, request.name, password_hash, code.clone(), now());

        if !self.users.insert(&user).await? {
            return Err(AppError::Conflict("Email already exists".into()));
        }
        tracing::info!(user_id = %user.id(), "registered user");

        self.mailer.send_verification_code(user.email().as_str(), code.as_str());
        Ok(UserDto::from(&user))
    }

    /// Подтверждение почты кодом; сразу открывает сессию.
    pub async fn verify_email(&self, email: &str, code: &str) -> AppResult<AuthResultDto> {
        let (_guard, mut user) = self.lock_by_email(email).await?;

        user.verify_email(code, now())?;
        if !self.users.update(&user).await? {
            return Err(AppError::not_found(ENTITY, email));
        }
        self.sign_in(&user).await
    }

    pub async fn resend_verification_code(&self, email: &str) -> AppResult<()> {
        let (_guard, mut user) = self.lock_by_email(email).await?;
        if user.is_email_verified() {
            return Err(AppError::not_found(ENTITY, email));
        }

        let now = now();
        if !user.can_reissue_code(now) {
            return Err(AppError::TooManyRequests);
        }
        let code = VerificationCode::generate();
        user.reissue_verification_code(code.clone(), now);
        if !self.users.update(&user).await? {
            return Err(AppError::not_found(ENTITY, email));
        }

        self.mailer.send_verification_code(user.email().as_str(), code.as_str());
        Ok(())
    }

    pub async fn login(&self, email: &str, password: String) -> AppResult<AuthResultDto> {
        let user = self.users.find_by_email(email).await?.ok_or(AppError::Unauthorized)?;
        if !self.hasher.verify(password, user.password_hash().clone()).await? {
            return Err(AppError::Unauthorized);
        }
        if !user.is_email_verified() {
            return Err(AppError::Forbidden);
        }
        self.sign_in(&user).await
    }

    /// Новая пара токенов по refresh-токену; старая сессия закрывается.
    pub async fn refresh(&self, refresh_token: &str) -> AppResult<TokenPairDto> {
        let session_id = Uuid::parse_str(refresh_token).map_err(|_| AppError::Unauthorized)?;
        // одна сессия — одна ротация: параллельный refresh тем же токеном получит 401
        let _guard = lock(session_id).await;
        let session = self.find_session(session_id).await?;

        self.sessions.delete(session_id).await?;
        self.issue(Session {
            session_id: Uuid::new_v4(),
            ..session
        })
        .await
    }

    pub async fn logout(&self, session_id: Uuid) -> AppResult<()> {
        Ok(self.sessions.delete(session_id).await?)
    }

    /// Проверка access-токена: подпись, срок и живая сессия того же пользователя.
    pub async fn authenticate(&self, access_token: &str) -> AppResult<Session> {
        let claims = self.tokens.decode(access_token).ok_or(AppError::Unauthorized)?;
        if claims.expires_at < Utc::now().timestamp() {
            return Err(AppError::Unauthorized);
        }
        let session = self.find_session(claims.session_id).await?;
        if session.user_id != claims.user_id {
            return Err(AppError::Unauthorized);
        }

        Ok(Session {
            user_id: claims.user_id,
            session_id: claims.session_id,
            verified: claims.verified,
        })
    }

    /// Нет сессии или хранилище недоступно — для клиента одинаково «не авторизован».
    async fn find_session(&self, session_id: Uuid) -> AppResult<Session> {
        match self.sessions.find(session_id).await {
            Ok(Some(session)) => Ok(session),
            Ok(None) => Err(AppError::Unauthorized),
            Err(error) => {
                tracing::warn!(%error, "failed to read session");
                Err(AppError::Unauthorized)
            }
        }
    }

    /// Пользователь по email под замком сущности: состояние перечитывается уже под замком.
    async fn lock_by_email(&self, email: &str) -> AppResult<(tokio::sync::MutexGuard<'static, ()>, User)> {
        let not_found = || AppError::not_found(ENTITY, email);
        let id = self.users.find_by_email(email).await?.ok_or_else(not_found)?.id();
        let guard = lock(id.into_inner()).await;
        let user = self.users.find_by_id(id).await?.ok_or_else(not_found)?;
        Ok((guard, user))
    }

    async fn sign_in(&self, user: &User) -> AppResult<AuthResultDto> {
        let tokens = self
            .issue(Session {
                user_id: user.id().into_inner(),
                session_id: Uuid::new_v4(),
                verified: user.is_email_verified(),
            })
            .await?;
        Ok(AuthResultDto {
            user: UserDto::from(user),
            tokens,
        })
    }

    async fn issue(&self, session: Session) -> AppResult<TokenPairDto> {
        self.sessions.save(&session, SESSION_TTL).await?;
        let access_token = self.tokens.encode(&TokenClaims {
            user_id: session.user_id,
            session_id: session.session_id,
            verified: session.verified,
            expires_at: (Utc::now() + SESSION_TTL).timestamp(),
        })?;

        Ok(TokenPairDto {
            access_token,
            refresh_token: session.session_id.to_string(),
            expires_in: SESSION_TTL.num_seconds().unsigned_abs(),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::Mutex;

    use async_trait::async_trait;
    use library_domain::users::PasswordHash;

    use super::*;
    use crate::ports::ExternalResult;
    use crate::testing::MockUsers;

    /// «Хэш» — сам пароль с префиксом.
    struct PlainHasher;

    #[async_trait]
    impl PasswordHasher for PlainHasher {
        async fn hash(&self, password: String) -> ExternalResult<PasswordHash> {
            Ok(PasswordHash::new(format!("hash:{password}")))
        }

        async fn verify(&self, password: String, hash: PasswordHash) -> ExternalResult<bool> {
            Ok(hash.as_str() == format!("hash:{password}"))
        }
    }

    /// Токен — `user:session:verified:exp`.
    struct PlainTokens;

    impl TokenCodec for PlainTokens {
        fn encode(&self, c: &TokenClaims) -> ExternalResult<String> {
            Ok(format!(
                "{}:{}:{}:{}",
                c.user_id, c.session_id, c.verified, c.expires_at
            ))
        }

        fn decode(&self, token: &str) -> Option<TokenClaims> {
            let mut parts = token.split(':');
            Some(TokenClaims {
                user_id: parts.next()?.parse().ok()?,
                session_id: parts.next()?.parse().ok()?,
                verified: parts.next()?.parse().ok()?,
                expires_at: parts.next()?.parse().ok()?,
            })
        }
    }

    #[derive(Default)]
    struct MemorySessions(Mutex<HashMap<Uuid, Session>>);

    #[async_trait]
    impl SessionStore for MemorySessions {
        async fn save(&self, session: &Session, _ttl: TimeDelta) -> ExternalResult<()> {
            self.0.lock().unwrap().insert(session.session_id, *session);
            Ok(())
        }

        async fn find(&self, session_id: Uuid) -> ExternalResult<Option<Session>> {
            Ok(self.0.lock().unwrap().get(&session_id).copied())
        }

        async fn delete(&self, session_id: Uuid) -> ExternalResult<()> {
            self.0.lock().unwrap().remove(&session_id);
            Ok(())
        }
    }

    #[derive(Default)]
    struct MemoryMailer(Mutex<Vec<(String, String)>>);

    impl Mailer for MemoryMailer {
        fn send_verification_code(&self, email: &str, code: &str) {
            self.0.lock().unwrap().push((email.into(), code.into()));
        }
    }

    struct Fixture {
        service: AuthService,
        sessions: Arc<MemorySessions>,
        mailer: Arc<MemoryMailer>,
    }

    fn fixture(users: MockUsers) -> Fixture {
        let sessions = Arc::new(MemorySessions::default());
        let mailer = Arc::new(MemoryMailer::default());
        let service = AuthService::new(
            Arc::new(users),
            Arc::new(PlainHasher),
            Arc::new(PlainTokens),
            sessions.clone(),
            mailer.clone(),
        );
        Fixture {
            service,
            sessions,
            mailer,
        }
    }

    fn user(verified: bool) -> User {
        let now = Utc::now();
        let mut user = User::register(
            Email::parse("user@example.com").unwrap(),
            "User".into(),
            PasswordHash::new("hash:secret".into()),
            VerificationCode::restore("123456".into()),
            now,
        );
        if verified {
            user.verify_email("123456", now).unwrap();
        }
        user
    }

    fn users_with(existing: User) -> MockUsers {
        let mut users = MockUsers::new();
        let by_id = existing.clone();
        users
            .expect_find_by_email()
            .returning(move |_| Ok(Some(existing.clone())));
        users.expect_find_by_id().returning(move |_| Ok(Some(by_id.clone())));
        users
    }

    #[tokio::test]
    async fn register_sends_code_and_returns_unverified_user() {
        let mut users = MockUsers::new();
        users
            .expect_insert()
            .withf(|user| user.password_hash().as_str() == "hash:secret")
            .returning(|_| Ok(true));
        let fixture = fixture(users);

        let dto = fixture
            .service
            .register(RegisterUser {
                name: "User".into(),
                email: "user@example.com".into(),
                password: "secret".into(),
            })
            .await
            .unwrap();

        assert!(!dto.is_email_verified && !dto.can_submit_materials);
        assert_eq!(dto.roles, ["user"]);
        let sent = fixture.mailer.0.lock().unwrap();
        assert_eq!(sent.len(), 1);
        assert_eq!(sent[0].0, "user@example.com");
    }

    #[tokio::test]
    async fn register_invalid_email_is_validation_error() {
        let mut users = MockUsers::new();
        users.expect_insert().never();

        let error = fixture(users)
            .service
            .register(RegisterUser {
                name: "User".into(),
                email: "nope".into(),
                password: "secret".into(),
            })
            .await
            .unwrap_err();

        assert!(matches!(error, AppError::Validation(errors) if errors.contains_key("email")));
    }

    #[tokio::test]
    async fn register_taken_email_is_conflict() {
        let mut users = MockUsers::new();
        users.expect_insert().returning(|_| Ok(false));

        let error = fixture(users)
            .service
            .register(RegisterUser {
                name: "User".into(),
                email: "user@example.com".into(),
                password: "secret".into(),
            })
            .await
            .unwrap_err();

        assert!(matches!(error, AppError::Conflict(_)));
    }

    #[tokio::test]
    async fn login_checks_password_then_verification() {
        let fixture = fixture(users_with(user(false)));

        let wrong = fixture.service.login("user@example.com", "wrong".into()).await;
        let unverified = fixture.service.login("user@example.com", "secret".into()).await;

        assert!(matches!(wrong, Err(AppError::Unauthorized)));
        assert!(matches!(unverified, Err(AppError::Forbidden)));
    }

    #[tokio::test]
    async fn login_unknown_email_is_unauthorized() {
        let mut users = MockUsers::new();
        users.expect_find_by_email().returning(|_| Ok(None));

        let error = fixture(users)
            .service
            .login("ghost@example.com", "secret".into())
            .await
            .unwrap_err();

        assert!(matches!(error, AppError::Unauthorized));
    }

    #[tokio::test]
    async fn login_opens_session_that_authenticates() {
        let fixture = fixture(users_with(user(true)));

        let result = fixture
            .service
            .login("user@example.com", "secret".into())
            .await
            .unwrap();
        let session = fixture.service.authenticate(&result.tokens.access_token).await.unwrap();

        assert_eq!(session.user_id, result.user.id);
        assert!(session.verified);
        assert_eq!(result.tokens.expires_in, 14 * 24 * 3600);
    }

    #[tokio::test]
    async fn refresh_rotates_session_and_logout_closes_it() {
        let fixture = fixture(users_with(user(true)));
        let first = fixture
            .service
            .login("user@example.com", "secret".into())
            .await
            .unwrap();

        let second = fixture.service.refresh(&first.tokens.refresh_token).await.unwrap();

        assert!(matches!(
            fixture.service.authenticate(&first.tokens.access_token).await,
            Err(AppError::Unauthorized)
        ));
        let session = fixture.service.authenticate(&second.access_token).await.unwrap();

        fixture.service.logout(session.session_id).await.unwrap();
        assert!(fixture.sessions.0.lock().unwrap().is_empty());
        assert!(matches!(
            fixture.service.refresh(&second.refresh_token).await,
            Err(AppError::Unauthorized)
        ));
    }

    #[tokio::test]
    async fn authenticate_rejects_expired_and_foreign_tokens() {
        let fixture = fixture(MockUsers::new());
        let session = Session {
            user_id: Uuid::new_v4(),
            session_id: Uuid::new_v4(),
            verified: true,
        };
        fixture.sessions.save(&session, SESSION_TTL).await.unwrap();
        let token = |user_id, expires_at| {
            PlainTokens
                .encode(&TokenClaims {
                    user_id,
                    session_id: session.session_id,
                    verified: true,
                    expires_at,
                })
                .unwrap()
        };
        let future = Utc::now().timestamp() + 60;

        assert!(
            fixture
                .service
                .authenticate(&token(session.user_id, future))
                .await
                .is_ok()
        );
        assert!(fixture.service.authenticate(&token(session.user_id, 0)).await.is_err());
        assert!(
            fixture
                .service
                .authenticate(&token(Uuid::new_v4(), future))
                .await
                .is_err()
        );
        assert!(fixture.service.authenticate("garbage").await.is_err());
    }

    #[tokio::test]
    async fn verify_email_opens_session() {
        let mut users = users_with(user(false));
        users
            .expect_update()
            .withf(|user| user.is_email_verified())
            .returning(|_| Ok(true));

        let result = fixture(users)
            .service
            .verify_email("user@example.com", "123456")
            .await
            .unwrap();

        assert!(result.user.is_email_verified && result.user.can_submit_materials);
    }

    #[tokio::test]
    async fn resend_too_soon_is_rate_limited() {
        let mut users = users_with(user(false));
        users.expect_update().never();

        let error = fixture(users)
            .service
            .resend_verification_code("user@example.com")
            .await
            .unwrap_err();

        assert!(matches!(error, AppError::TooManyRequests));
    }

    #[tokio::test]
    async fn resend_for_verified_user_is_not_found() {
        let error = fixture(users_with(user(true)))
            .service
            .resend_verification_code("user@example.com")
            .await
            .unwrap_err();

        assert!(matches!(error, AppError::NotFound { .. }));
    }
}
