use crate::domain::tier::Tier;

pub struct CreateSubscriptionParams {
    pub user_id: domain::domain_user::UserId,
    pub tier: Tier,
    pub expires_at: Option<String>,
}
