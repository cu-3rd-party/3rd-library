use chrono::TimeDelta;
use rand::Rng;

/// Код подтверждения почты: 6 случайных цифр.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerificationCode(String);

impl VerificationCode {
    pub const LENGTH: usize = 6;
    /// Сколько код действителен.
    pub const TTL: TimeDelta = TimeDelta::hours(24);
    /// Не чаще одного нового кода за этот интервал.
    pub const RESEND_INTERVAL: TimeDelta = TimeDelta::minutes(1);

    pub fn generate() -> Self {
        let mut rng = rand::thread_rng();
        Self(
            (0..Self::LENGTH)
                .map(|_| char::from(b'0' + rng.gen_range(0..10u8)))
                .collect(),
        )
    }

    /// Восстановление из хранилища.
    pub fn restore(value: String) -> Self {
        Self(value)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_makes_six_digits() {
        let code = VerificationCode::generate();
        assert_eq!(code.as_str().len(), VerificationCode::LENGTH);
        assert!(code.as_str().chars().all(|c| c.is_ascii_digit()));
    }
}
