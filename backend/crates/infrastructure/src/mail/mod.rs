//! Письма пользователям. Без фичи `smtp` (или без настроек SMTP) письма не уходят, код пишется в лог.

#[cfg(feature = "smtp")]
mod smtp;

use std::sync::Arc;

use library_application::ports::Mailer;

use crate::InfrastructureConfig;

#[cfg(feature = "smtp")]
pub use self::smtp::SmtpMailer;

pub(crate) fn mailer(config: &InfrastructureConfig) -> Arc<dyn Mailer> {
    #[cfg(feature = "smtp")]
    if let Some(smtp) = &config.smtp {
        return Arc::new(SmtpMailer::new(smtp.clone()));
    }
    #[cfg(not(feature = "smtp"))]
    let _ = config;
    tracing::warn!("SMTP is not configured: verification codes are only logged");
    Arc::new(LogMailer)
}

/// Для локальной разработки: письмо не отправляется, код подтверждения — в логе.
pub struct LogMailer;

impl Mailer for LogMailer {
    fn send_verification_code(&self, email: &str, code: &str) {
        tracing::info!(%email, %code, "verification code (email is not sent)");
    }
}
