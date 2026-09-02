use crate::domain::db::db_purchased_product::{DbPurchasedProduct, PurchasedProductId};
use crate::domain::service_error::ServiceError;
use crate::purchased_products::create_purchased_product_params::CreatePurchasedProductParams;
use crate::purchased_products::data_provider::purchased_products_data_provider::PurchasedProductsDataProvider;
use crate::purchased_products::purchased_products_provider::PurchasedProductsProvider;
use async_trait::async_trait;
use std::sync::Arc;

pub struct PurchasedProductsProviderService {
    data_provider: Arc<dyn PurchasedProductsDataProvider>,
}
impl PurchasedProductsProviderService {
    pub fn new(data_provider: Arc<dyn PurchasedProductsDataProvider>) -> Self {
        Self { data_provider }
    }
}
#[async_trait]
impl PurchasedProductsProvider for PurchasedProductsProviderService {
    async fn create_purchased_product(
        &self,
        params: CreatePurchasedProductParams,
    ) -> Result<DbPurchasedProduct, ServiceError> {
        self.data_provider
            .create_purchased_product(params)
            .await
            .map_err(ServiceError::from)
    }
    async fn get_purchased_product(
        &self,
        id: PurchasedProductId,
    ) -> Result<Option<DbPurchasedProduct>, ServiceError> {
        self.data_provider
            .get_purchased_product(id)
            .await
            .map_err(ServiceError::from)
    }
    async fn list_by_user(
        &self,
        user_id: domain::domain_user::UserId,
    ) -> Result<Vec<DbPurchasedProduct>, ServiceError> {
        self.data_provider
            .list_by_user(user_id)
            .await
            .map_err(ServiceError::from)
    }
    async fn delete_purchased_product(&self, id: PurchasedProductId) -> Result<bool, ServiceError> {
        self.data_provider
            .delete_purchased_product(id)
            .await
            .map_err(ServiceError::from)
    }
}
