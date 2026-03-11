use crate::domain::db::db_organization::OrganizationId;
use crate::domain::Tier;

pub struct CreateSubscriptionCheckoutParams {
    pub org_id: OrganizationId,
    pub org_slug: String,
    pub tier: Tier,
    pub success_url: Option<String>,
    pub cancel_url: Option<String>,
}
