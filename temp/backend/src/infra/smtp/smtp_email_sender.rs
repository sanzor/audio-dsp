use async_trait::async_trait;
use lettre::{
    message::header::ContentType, transport::smtp::authentication::Credentials, AsyncSmtpTransport,
    AsyncTransport, Message, Tokio1Executor,
};
use tracing::{error, info};

use crate::auth::email_sender::EmailSender;
use crate::infra::smtp::smtp_email_sender_config::SmtpEmailSenderConfig;

pub struct SmtpEmailSender {
    mailer: AsyncSmtpTransport<Tokio1Executor>,
    from_address: String,
    verification_base_url: String,
}

impl SmtpEmailSender {
    pub fn new(config: SmtpEmailSenderConfig) -> Result<Self, String> {
        let builder = if config.use_starttls {
            AsyncSmtpTransport::<Tokio1Executor>::relay(&config.host)
                .map_err(|e| format!("failed to create SMTP transport: {e}"))?
                .port(config.port)
        } else {
            AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&config.host).port(config.port)
        };

        let builder = match (config.username.as_ref(), config.password.as_ref()) {
            (Some(username), Some(password))
                if !username.trim().is_empty() && !password.trim().is_empty() =>
            {
                builder.credentials(Credentials::new(username.to_string(), password.to_string()))
            }
            _ => builder,
        };

        let mailer = builder.build();

        Ok(Self {
            mailer,
            from_address: config.from_address,
            verification_base_url: config.verification_base_url,
        })
    }
}

#[async_trait]
impl EmailSender for SmtpEmailSender {
    async fn send_email(&self, to: &str, subject: &str, body: &str) -> Result<(), String> {
        let email = Message::builder()
            .from(
                self.from_address
                    .parse()
                    .map_err(|e| format!("invalid from address: {e}"))?,
            )
            .to(to.parse().map_err(|e| format!("invalid to address: {e}"))?)
            .subject(subject)
            .header(ContentType::TEXT_PLAIN)
            .body(body.to_string())
            .map_err(|e| format!("failed to build email: {e}"))?;

        self.mailer.send(email).await.map_err(|e| {
            error!(error = %e, to = %to, subject = %subject, "failed to send email");
            format!("failed to send email: {e}")
        })?;

        info!(to = %to, subject = %subject, "email sent");
        Ok(())
    }
}
