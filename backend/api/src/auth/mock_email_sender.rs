use super::email_sender::EmailSender;
use tracing::info;

pub struct MockEmailSender;

#[async_trait::async_trait]
impl EmailSender for MockEmailSender {
    async fn send_email(&self, to: &str, subject: &str, body: &str) -> Result<(), String> {
        info!(to, subject, body, "mock email sent");
        Ok(())
    }
}
