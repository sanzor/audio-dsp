use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

pub type ProductId = i64;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, FromRow)]
pub struct DbProduct {
    pub id: ProductId,
    pub name: String,
    pub description: Option<String>,
    pub token_amount: i64,
    pub price_cents: i64,
    pub currency: String,
    pub stripe_price_id: Option<String>,
    pub stripe_product_id: Option<String>,
    pub is_active: bool,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}
