use crate::domain::db::db_organization::OrganizationId;
use crate::domain::Tier;

#[derive(Clone, Debug)]
pub struct CreateSubscriptionParams {
    pub org_id: OrganizationId,
    pub tier: Tier,
    pub stripe_subscription_id: Option<String>,
    pub status: String,
    pub current_period_end: Option<String>,
}
