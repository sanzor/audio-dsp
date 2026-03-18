use crate::domain::tier::Tier;

pub struct CreateProductParams {
    pub name: String,
    pub description: Option<String>,
    pub tier: Tier,
    pub price_cents: i64,
    pub currency: String,
}
