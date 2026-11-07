use chrono::{DateTime, Utc};

use super::{Email, VerificationCode};
use crate::DomainError;
use crate::common::{Audit, entity_id};

entity_id!(UserId);

pub const ROLE_USER: &str = "user";
pub const ROLE_MODERATOR: &str = "moderator";

/// Хэш пароля в формате PHC (считает порт `PasswordHasher` в приложении).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PasswordHash(String);

impl PasswordHash {
    pub fn new(value: String) -> Self {
        Self(value)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Агрегат «пользователь». Поля приватные: состояние меняется только через методы.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    id: UserId,
    email: Email,
    password_hash: PasswordHash,
    name: String,
    bio: String,
    roles: Vec<String>,
    is_email_verified: bool,
    verification_code: Option<VerificationCode>,
    code_issued_at: Option<DateTime<Utc>>,
    code_expires_at: Option<DateTime<Utc>>,
    audit: Audit,
}

/// Состояние пользователя из хранилища — для [`User::restore`].
#[derive(Debug, Clone)]
pub struct UserData {
    pub id: UserId,
    pub email: Email,
    pub password_hash: PasswordHash,
    pub name: String,
    pub bio: String,
    pub roles: Vec<String>,
    pub is_email_verified: bool,
    pub verification_code: Option<VerificationCode>,
    pub code_issued_at: Option<DateTime<Utc>>,
    pub code_expires_at: Option<DateTime<Utc>>,
    pub audit: Audit,
}

impl User {
    /// Новый пользователь с неподтверждённой почтой и выданным кодом подтверждения.
    pub fn register(
        email: Email,
        name: String,
        password_hash: PasswordHash,
        code: VerificationCode,
        now: DateTime<Utc>,
    ) -> Self {
        Self {
            id: UserId::new(),
            email,
            password_hash,
            name,
            bio: String::new(),
            roles: vec![ROLE_USER.to_owned()],
            is_email_verified: false,
            verification_code: Some(code),
            code_issued_at: Some(now),
            code_expires_at: Some(now + VerificationCode::TTL),
            audit: Audit::new(),
        }
    }

    /// Восстановление из хранилища (без проверок и без изменения аудита).
    pub fn restore(data: UserData) -> Self {
        Self {
            id: data.id,
            email: data.email,
            password_hash: data.password_hash,
            name: data.name,
            bio: data.bio,
            roles: data.roles,
            is_email_verified: data.is_email_verified,
            verification_code: data.verification_code,
            code_issued_at: data.code_issued_at,
            code_expires_at: data.code_expires_at,
            audit: data.audit,
        }
    }

    /// Подтверждение почты кодом из письма.
    pub fn verify_email(&mut self, code: &str, now: DateTime<Utc>) -> Result<(), DomainError> {
        if self.is_email_verified {
            return Err(DomainError::Conflict("Email already verified".into()));
        }
        if self
            .verification_code
            .as_ref()
            .is_none_or(|expected| expected.as_str() != code)
        {
            return Err(DomainError::validation("code", "invalid verification code"));
        }
        if self.code_expires_at.is_some_and(|expires| expires < now) {
            return Err(DomainError::validation("code", "verification code has expired"));
        }

        self.is_email_verified = true;
        self.verification_code = None;
        self.code_expires_at = None;
        self.audit.touch();
        Ok(())
    }

    /// Можно ли выдать новый код: прошлый выдан не позже [`VerificationCode::RESEND_INTERVAL`] назад.
    pub fn can_reissue_code(&self, now: DateTime<Utc>) -> bool {
        self.code_issued_at
            .is_none_or(|issued| now - issued >= VerificationCode::RESEND_INTERVAL)
    }

    pub fn reissue_verification_code(&mut self, code: VerificationCode, now: DateTime<Utc>) {
        self.verification_code = Some(code);
        self.code_issued_at = Some(now);
        self.code_expires_at = Some(now + VerificationCode::TTL);
        self.audit.touch();
    }

    /// Правка профиля: `None` — поле не меняется.
    pub fn update_profile(&mut self, name: Option<String>, bio: Option<String>) {
        if let Some(name) = name {
            self.name = name;
        }
        if let Some(bio) = bio {
            self.bio = bio;
        }
        self.audit.touch();
    }

    /// Роли из хранилища: их выдают в БД в обход бэкенда, и состояние в памяти с ними сверяется.
    pub fn sync_roles(&mut self, roles: Vec<String>) {
        self.roles = roles;
    }

    pub fn is_moderator(&self) -> bool {
        self.roles.iter().any(|role| role == ROLE_MODERATOR)
    }

    /// Загружать материалы может только пользователь с подтверждённой почтой.
    pub fn can_submit_materials(&self) -> bool {
        self.is_email_verified
    }

    pub fn id(&self) -> UserId {
        self.id
    }

    pub fn email(&self) -> &Email {
        &self.email
    }

    pub fn password_hash(&self) -> &PasswordHash {
        &self.password_hash
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn bio(&self) -> &str {
        &self.bio
    }

    pub fn roles(&self) -> &[String] {
        &self.roles
    }

    pub fn is_email_verified(&self) -> bool {
        self.is_email_verified
    }

    pub fn verification_code(&self) -> Option<&VerificationCode> {
        self.verification_code.as_ref()
    }

    pub fn code_issued_at(&self) -> Option<DateTime<Utc>> {
        self.code_issued_at
    }

    pub fn code_expires_at(&self) -> Option<DateTime<Utc>> {
        self.code_expires_at
    }

    pub fn audit(&self) -> &Audit {
        &self.audit
    }
}

#[cfg(test)]
mod tests {
    use chrono::TimeDelta;

    use super::*;

    fn registered(now: DateTime<Utc>) -> User {
        User::register(
            Email::parse("user@example.com").unwrap(),
            "User".into(),
            PasswordHash::new("hash".into()),
            VerificationCode::restore("123456".into()),
            now,
        )
    }

    #[test]
    fn register_is_unverified_plain_user() {
        let user = registered(Utc::now());

        assert!(!user.is_email_verified());
        assert!(!user.can_submit_materials());
        assert!(!user.is_moderator());
        assert_eq!(user.roles(), [ROLE_USER]);
    }

    #[test]
    fn verify_email_with_right_code() {
        let now = Utc::now();
        let mut user = registered(now);

        user.verify_email("123456", now).unwrap();

        assert!(user.is_email_verified());
        assert!(user.verification_code().is_none());
    }

    #[test]
    fn verify_email_rejects_wrong_expired_and_repeated() {
        let now = Utc::now();
        let mut user = registered(now);

        assert!(matches!(
            user.verify_email("000000", now),
            Err(DomainError::Validation { .. })
        ));
        assert!(matches!(
            user.verify_email("123456", now + TimeDelta::hours(25)),
            Err(DomainError::Validation { .. })
        ));

        user.verify_email("123456", now).unwrap();
        assert!(matches!(
            user.verify_email("123456", now),
            Err(DomainError::Conflict(_))
        ));
    }

    #[test]
    fn reissue_is_rate_limited_from_last_code() {
        let now = Utc::now();
        let mut user = registered(now);

        assert!(!user.can_reissue_code(now + TimeDelta::seconds(30)));
        let later = now + TimeDelta::minutes(2);
        assert!(user.can_reissue_code(later));

        user.reissue_verification_code(VerificationCode::restore("654321".into()), later);

        assert!(!user.can_reissue_code(later + TimeDelta::seconds(30)));
        assert_eq!(user.verification_code().unwrap().as_str(), "654321");
    }

    #[test]
    fn update_profile_keeps_missing_fields() {
        let mut user = registered(Utc::now());

        user.update_profile(None, Some("bio".into()));

        assert_eq!((user.name(), user.bio()), ("User", "bio"));
    }
}
