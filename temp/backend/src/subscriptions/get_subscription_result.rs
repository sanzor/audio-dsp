use crate::subscriptions::DbSubscription;

#[derive(Clone, Debug)]
pub struct GetSubscriptionResult {
    pub subscription: DbSubscription,
}
