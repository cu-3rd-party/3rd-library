use crate::http::ApiContext;
use ammonia::clean;
use mail_send::SmtpClientBuilder;
use mail_send::mail_builder::MessageBuilder;
use rust_embed::RustEmbed;
use std::sync::OnceLock;

#[derive(RustEmbed, Clone)]
#[folder = "src/smtp/html"]
struct Html;

#[derive(RustEmbed, Clone)]
#[folder = "src/smtp/txt"]
struct Txt;

static VERIFICATION_CODE_HTML: OnceLock<String> = OnceLock::new();
static VERIFICATION_CODE_TXT: OnceLock<String> = OnceLock::new();

pub async fn send_verification_code(
    context: ApiContext,
    email: String,
    verification_code: String,
) -> Result<(), String> {
    let template_html = VERIFICATION_CODE_HTML.get_or_init(|| {
        Html::get("./verification_code.html")
            .and_then(|html| String::from_utf8(html.data.to_vec()).ok())
            .unwrap_or_default()
    });
    let template_txt = VERIFICATION_CODE_TXT.get_or_init(|| {
        Html::get("./verification_code.txt")
            .and_then(|html| String::from_utf8(html.data.to_vec()).ok())
            .unwrap_or_default()
    });
    let message = MessageBuilder::new()
        .from(context.config.smtp.user.clone())
        .to(email.clone())
        .subject("Код подтверждения")
        .text_body(template_txt.replace("{verification_code}", &verification_code))
        .html_body(template_html.replace("{verification_code}", &clean(&verification_code)));

    SmtpClientBuilder::new(context.config.smtp.host.clone(), context.config.smtp.port)?
        .implicit_tls(true)
        .credentials((
            context.config.smtp.user.clone(),
            context.config.smtp.password.clone(),
        ))
        .connect()
        .await
        .map_err(|e| e.to_string())?
        .send(message)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}
