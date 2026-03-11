use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

use crate::domain::db::db_organization::OrganizationId;
use crate::domain::Tier;

pub type SubscriptionId = i64;
pub type StripeSubscriptionId = String;
#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, FromRow)]
pub struct DbSubscription {
    pub id: SubscriptionId,
    pub org_id: OrganizationId,
    pub tier: Tier,
    pub stripe_subscription_id: Option<StripeSubscriptionId>,
    pub status: String,
    pub current_period_end: Option<String>,
}
