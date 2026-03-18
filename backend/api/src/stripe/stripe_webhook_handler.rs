use async_trait::async_trait;
use stripe::Event;

#[async_trait]
pub trait StripeWebhookHandler: Send + Sync {
    async fn handle_event(&self, event: Event);
}
