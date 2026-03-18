use async_trait::async_trait;
use crate::domain::data_error::DataError;
use crate::domain::db::db_product::{DbProduct, ProductId};
use crate::products::create_product_params::CreateProductParams;
use crate::products::update_product_params::UpdateProductParams;

#[async_trait]
pub trait ProductsDataProvider: Send + Sync {
    async fn create_product(&self, params: CreateProductParams) -> Result<DbProduct, DataError>;
    async fn get_product(&self, id: ProductId) -> Result<Option<DbProduct>, DataError>;
    async fn update_product(&self, id: ProductId, params: UpdateProductParams) -> Result<Option<DbProduct>, DataError>;
    async fn delete_product(&self, id: ProductId) -> Result<bool, DataError>;
    async fn list_products(&self, active_only: bool) -> Result<Vec<DbProduct>, DataError>;
}
