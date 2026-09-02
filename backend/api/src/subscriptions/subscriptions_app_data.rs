use crate::subscriptions::subscriptions_provider::SubscriptionsProvider;
use std::sync::Arc;

#[derive(Clone)]
pub struct SubscriptionsAppData {
    pub subscriptions_provider: Arc<dyn SubscriptionsProvider>,
}
