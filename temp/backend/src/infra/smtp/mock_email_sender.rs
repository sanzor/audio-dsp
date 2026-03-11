use async_trait::async_trait;
use tracing::info;

use crate::auth::email_sender::EmailSender;

pub struct MockEmailSender;

impl MockEmailSender {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl EmailSender for MockEmailSender {
    async fn send_email(&self, to: &str, subject: &str, body: &str) -> Result<(), String> {
        info!(to = %to, subject = %subject, body = %body, "mock email sent");
        Ok(())
    }
}
