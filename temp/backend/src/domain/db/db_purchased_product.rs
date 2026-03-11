use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

use crate::domain::db::db_organization::OrganizationId;
use crate::domain::db::db_product::ProductId;

pub type PurchasedProductId = i64;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, FromRow)]
pub struct DbPurchasedProduct {
    pub id: PurchasedProductId,
    pub org_id: OrganizationId,
    pub product_id: ProductId,
    pub tokens_granted: i64,
    pub stripe_payment_intent_id: Option<String>,
    pub purchased_at: Option<String>,
}
