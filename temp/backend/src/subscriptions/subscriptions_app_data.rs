use std::sync::Arc;

use crate::subscriptions::subscriptions_provider::SubscriptionsProvider;

#[derive(Clone)]
pub struct SubscriptionsAppData {
    pub subscriptions_provider: Arc<dyn SubscriptionsProvider>,
}
