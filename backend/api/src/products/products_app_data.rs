use crate::products::products_provider::ProductsProvider;
use std::sync::Arc;

#[derive(Clone)]
pub struct ProductsAppData {
    pub products_provider: Arc<dyn ProductsProvider>,
}
