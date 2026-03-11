use std::sync::Arc;

use crate::products::products_provider::ProductsProvider;

#[derive(Clone)]
pub struct ProductsAppData {
    pub products_provider: Arc<dyn ProductsProvider>,
}
