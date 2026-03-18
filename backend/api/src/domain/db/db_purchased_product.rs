use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

pub type PurchasedProductId = i64;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, FromRow)]
pub struct DbPurchasedProduct {
    pub id: PurchasedProductId,
    pub user_id: String,
    pub product_id: i64,
    pub purchased_at: String,
}
