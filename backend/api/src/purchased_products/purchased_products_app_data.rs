use crate::purchased_products::purchased_products_provider::PurchasedProductsProvider;
use std::sync::Arc;

#[derive(Clone)]
pub struct PurchasedProductsAppData {
    pub purchased_products_provider: Arc<dyn PurchasedProductsProvider>,
}
