use crate::domain::Tier;

#[derive(Clone, Debug)]
pub struct UpdateSubscriptionParams {
    pub tier: Option<Tier>,
    pub stripe_subscription_id: Option<String>,
    pub status: Option<String>,
    pub current_period_end: Option<String>,
}
