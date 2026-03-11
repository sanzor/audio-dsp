use crate::subscriptions::DbSubscription;

#[derive(Clone, Debug)]
pub struct UpdateSubscriptionResult {
    pub subscription: DbSubscription,
}
