use crate::domain::db::db_product::{DbProduct, ProductId};
use crate::domain::service_error::ServiceError;
use crate::products::create_product_params::CreateProductParams;
use crate::products::data_provider::products_data_provider::ProductsDataProvider;
use crate::products::products_provider::ProductsProvider;
use crate::products::update_product_params::UpdateProductParams;
use async_trait::async_trait;
use std::sync::Arc;

pub struct ProductsProviderService {
    data_provider: Arc<dyn ProductsDataProvider>,
}
impl ProductsProviderService {
    pub fn new(data_provider: Arc<dyn ProductsDataProvider>) -> Self {
        Self { data_provider }
    }
}
#[async_trait]
impl ProductsProvider for ProductsProviderService {
    async fn create_product(&self, params: CreateProductParams) -> Result<DbProduct, ServiceError> {
        self.data_provider
            .create_product(params)
            .await
            .map_err(ServiceError::from)
    }
    async fn get_product(&self, id: ProductId) -> Result<Option<DbProduct>, ServiceError> {
        self.data_provider
            .get_product(id)
            .await
            .map_err(ServiceError::from)
    }
    async fn update_product(
        &self,
        id: ProductId,
        params: UpdateProductParams,
    ) -> Result<Option<DbProduct>, ServiceError> {
        self.data_provider
            .update_product(id, params)
            .await
            .map_err(ServiceError::from)
    }
    async fn delete_product(&self, id: ProductId) -> Result<bool, ServiceError> {
        self.data_provider
            .delete_product(id)
            .await
            .map_err(ServiceError::from)
    }
    async fn list_products(&self, active_only: bool) -> Result<Vec<DbProduct>, ServiceError> {
        self.data_provider
            .list_products(active_only)
            .await
            .map_err(ServiceError::from)
    }
}
