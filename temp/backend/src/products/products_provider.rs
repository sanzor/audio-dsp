use async_trait::async_trait;

use crate::domain::db::db_product::ProductId;
use crate::domain::service_error::ServiceError;
use crate::products::create_product_params::CreateProductParams;
use crate::products::update_product_params::UpdateProductParams;
use crate::products::DbProduct;

#[async_trait]
pub trait ProductsProvider: Send + Sync {
    async fn create_product(&self, params: CreateProductParams) -> Result<DbProduct, ServiceError>;
    async fn update_product(
        &self,
        id: ProductId,
        params: UpdateProductParams,
    ) -> Result<Option<DbProduct>, ServiceError>;
    async fn delete_product(&self, id: ProductId) -> Result<bool, ServiceError>;
    async fn get_product(&self, id: ProductId) -> Result<Option<DbProduct>, ServiceError>;
    async fn list_products(&self) -> Result<Vec<DbProduct>, ServiceError>;

    async fn activate_product(&self, id: ProductId) -> Result<(), ServiceError>;
    async fn deactivate_product(&self, id: ProductId) -> Result<(), ServiceError>;
}
