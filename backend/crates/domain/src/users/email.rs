use std::fmt;
use std::sync::OnceLock;

use regex::Regex;

use crate::DomainError;

static EMAIL_REGEX: OnceLock<Regex> = OnceLock::new();

/// Адрес почты пользователя (проверяется при регистрации).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Email(String);

impl Email {
    pub fn parse(value: impl AsRef<str>) -> Result<Self, DomainError> {
        let value = value.as_ref();
        let regex = EMAIL_REGEX.get_or_init(|| {
            Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").expect("valid email regex")
        });
        if !regex.is_match(value) {
            return Err(DomainError::validation("email", "must be a valid email address"));
        }
        Ok(Self(value.to_owned()))
    }

    /// Восстановление из хранилища (без проверки: старые записи не должны ломать вход).
    pub fn restore(value: String) -> Self {
        Self(value)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_accepts_regular_address() {
        assert_eq!(
            Email::parse("a.b+c@mail.example.ru").unwrap().as_str(),
            "a.b+c@mail.example.ru"
        );
    }

    #[test]
    fn parse_rejects_garbage() {
        for value in ["", "not-an-email", "a@b", "a b@c.ru"] {
            assert!(Email::parse(value).is_err(), "{value}");
        }
    }
}
