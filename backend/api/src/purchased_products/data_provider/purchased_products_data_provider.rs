use crate::domain::data_error::DataError;
use crate::domain::db::db_purchased_product::{DbPurchasedProduct, PurchasedProductId};
use crate::purchased_products::create_purchased_product_params::CreatePurchasedProductParams;
use async_trait::async_trait;

#[async_trait]
pub trait PurchasedProductsDataProvider: Send + Sync {
    async fn create_purchased_product(
        &self,
        params: CreatePurchasedProductParams,
    ) -> Result<DbPurchasedProduct, DataError>;
    async fn get_purchased_product(
        &self,
        id: PurchasedProductId,
    ) -> Result<Option<DbPurchasedProduct>, DataError>;
    async fn list_by_user(
        &self,
        user_id: domain::domain_user::UserId,
    ) -> Result<Vec<DbPurchasedProduct>, DataError>;
    async fn delete_purchased_product(&self, id: PurchasedProductId) -> Result<bool, DataError>;
}
