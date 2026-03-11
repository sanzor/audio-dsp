use crate::domain::db::db_organization::OrganizationId;
use crate::domain::db::db_product::ProductId;

#[derive(Clone, Debug)]
pub struct CreatePurchasedProductParams {
    pub org_id: OrganizationId,
    pub product_id: ProductId,
    pub tokens_granted: i64,
    pub stripe_payment_intent_id: Option<String>,
}
