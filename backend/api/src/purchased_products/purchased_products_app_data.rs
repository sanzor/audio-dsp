use std::sync::Arc;
use crate::purchased_products::purchased_products_provider::PurchasedProductsProvider;

#[derive(Clone)]
pub struct PurchasedProductsAppData {
    pub purchased_products_provider: Arc<dyn PurchasedProductsProvider>,
}
