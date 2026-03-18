use std::sync::Arc;

use crate::stripe::{stripe_service::StripeService, stripe_webhook_handler::StripeWebhookHandler};

pub struct StripeWebhookAppData {
    pub stripe: Arc<StripeService>,
    pub webhook_service: Arc<dyn StripeWebhookHandler>,
}
