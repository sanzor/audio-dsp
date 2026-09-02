use crate::domain::tier::Tier;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

pub type ProductId = i32;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, FromRow)]
pub struct DbProduct {
    pub id: ProductId,
    pub name: String,
    pub description: Option<String>,
    pub tier: Tier,
    pub price_cents: i64,
    pub currency: String,
    pub is_active: bool,
    pub created_at: String,
}
