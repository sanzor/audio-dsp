use async_trait::async_trait;
use crate::domain::db::db_purchased_product::{DbPurchasedProduct, PurchasedProductId};
use crate::domain::service_error::ServiceError;
use crate::purchased_products::create_purchased_product_params::CreatePurchasedProductParams;

#[async_trait]
pub trait PurchasedProductsProvider: Send + Sync {
    async fn create_purchased_product(&self, params: CreatePurchasedProductParams) -> Result<DbPurchasedProduct, ServiceError>;
    async fn get_purchased_product(&self, id: PurchasedProductId) -> Result<Option<DbPurchasedProduct>, ServiceError>;
    async fn list_by_user(&self, user_id: domain::domain_user::UserId) -> Result<Vec<DbPurchasedProduct>, ServiceError>;
    async fn delete_purchased_product(&self, id: PurchasedProductId) -> Result<bool, ServiceError>;
}
