#[derive(Clone, Debug)]
pub struct CreateProductParams {
    pub name: String,
    pub description: Option<String>,
    pub token_amount: i64,
    pub price_cents: i64,
    pub currency: String,
    pub stripe_price_id: Option<String>,
}
