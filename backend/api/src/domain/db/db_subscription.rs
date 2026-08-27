use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use crate::domain::tier::Tier;

pub type SubscriptionId = i32;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, FromRow)]
pub struct DbSubscription {
    pub id: SubscriptionId,
    pub user_id: domain::domain_user::UserId,
    pub tier: Tier,
    pub is_active: bool,
    pub started_at: String,
    pub expires_at: Option<String>,
}
