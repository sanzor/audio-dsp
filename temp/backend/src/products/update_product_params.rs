#[derive(Clone, Debug)]
pub struct UpdateProductParams {
    pub name: Option<String>,
    pub description: Option<String>,
    pub token_amount: Option<i64>,
    pub price_cents: Option<i64>,
    pub stripe_price_id: Option<String>,
    pub is_active: Option<bool>,
}
