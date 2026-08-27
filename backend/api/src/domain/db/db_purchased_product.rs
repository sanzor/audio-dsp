use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

pub type PurchasedProductId = i32;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, FromRow)]
pub struct DbPurchasedProduct {
    pub id: PurchasedProductId,
    pub user_id: domain::domain_user::UserId,
    pub product_id: i32,
    pub purchased_at: String,
}
