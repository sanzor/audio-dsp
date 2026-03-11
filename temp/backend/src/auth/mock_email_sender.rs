use async_trait::async_trait;
use tracing::info;

use crate::auth::email_sender::EmailSender;

pub struct MockEmailSender {
    from_address: String,
    verification_base_url: String,
}

impl MockEmailSender {
    pub fn new(from_address: String, verification_base_url: String) -> Self {
        Self {
            from_address,
            verification_base_url,
        }
    }
}

#[async_trait]
impl EmailSender for MockEmailSender {
    async fn send_email(&self, to: &str, subject: &str, body: &str) -> Result<(), String> {
        info!(
            to = %to,
            from = %self.from_address,
            subject = %subject,
            body = %body,
            "mock email"
        );
        Ok(())
    }
}
