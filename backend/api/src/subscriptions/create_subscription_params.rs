use crate::domain::tier::Tier;

pub struct CreateSubscriptionParams {
    pub user_id: i32,
    pub tier: Tier,
    pub expires_at: Option<String>,
}
