use crate::domain::tier::Tier;

pub struct UpdateProductParams {
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub tier: Option<Tier>,
    pub price_cents: Option<i64>,
    pub currency: Option<String>,
    pub is_active: Option<bool>,
}
