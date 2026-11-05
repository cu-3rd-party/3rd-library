use std::sync::{Arc, LazyLock};

use ammonia::clean;
use library_application::ports::Mailer;
use mail_send::SmtpClientBuilder;
use mail_send::mail_builder::MessageBuilder;
use rust_embed::RustEmbed;

use crate::SmtpConfig;

#[derive(RustEmbed)]
#[folder = "src/mail/html"]
struct Html;

#[derive(RustEmbed)]
#[folder = "src/mail/txt"]
struct Txt;

const VERIFICATION_CODE_HTML_FALLBACK: &str = r#"<html><body><p>Привет,</p><p>Вы запросили код авторизации, вот ваш код. Этот код истечет через <strong>10 минут</strong>.</p><p><code>{verification_code}</code></p></body></html>"#;
const VERIFICATION_CODE_TXT_FALLBACK: &str =
    "Привет,\nВы запросили код авторизации, вот ваш код. Этот код истечет через 10 минут.\n{verification_code}";

static VERIFICATION_CODE_HTML: LazyLock<String> =
    LazyLock::new(|| template::<Html>("verification_code.html", VERIFICATION_CODE_HTML_FALLBACK));
static VERIFICATION_CODE_TXT: LazyLock<String> =
    LazyLock::new(|| template::<Txt>("verification_code.txt", VERIFICATION_CODE_TXT_FALLBACK));

fn template<E: RustEmbed>(name: &str, fallback: &str) -> String {
    E::get(name)
        .and_then(|file| String::from_utf8(file.data.into_owned()).ok())
        .unwrap_or_else(|| fallback.to_owned())
}

/// SMTP с неявным TLS; письмо отправляется в фоновой задаче.
pub struct SmtpMailer {
    config: Arc<SmtpConfig>,
}

impl SmtpMailer {
    pub fn new(config: SmtpConfig) -> Self {
        Self {
            config: Arc::new(config),
        }
    }
}

impl Mailer for SmtpMailer {
    fn send_verification_code(&self, email: &str, code: &str) {
        let config = self.config.clone();
        let email = email.to_owned();
        let code = code.to_owned();
        tokio::spawn(async move {
            if let Err(error) = send_verification_code(&config, &email, &code).await {
                tracing::error!(%error, %email, "failed to send verification code");
            }
        });
    }
}

async fn send_verification_code(config: &SmtpConfig, email: &str, code: &str) -> Result<(), String> {
    let message = MessageBuilder::new()
        .from(config.user.clone())
        .to(email.to_owned())
        .subject("Код подтверждения")
        .text_body(VERIFICATION_CODE_TXT.replace("{verification_code}", code))
        .html_body(VERIFICATION_CODE_HTML.replace("{verification_code}", &clean(code)));

    SmtpClientBuilder::new(config.host.clone(), config.port)?
        .implicit_tls(true)
        .credentials((config.user.clone(), config.password.clone()))
        .connect()
        .await
        .map_err(|error| error.to_string())?
        .send(message)
        .await
        .map_err(|error| error.to_string())
}
