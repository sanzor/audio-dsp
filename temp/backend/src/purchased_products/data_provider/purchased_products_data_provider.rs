use async_trait::async_trait;

use crate::domain::data_error::DataError;
use crate::domain::db::db_organization::OrganizationId;
use crate::domain::db::db_product::ProductId;
use crate::domain::db::db_purchased_product::PurchasedProductId;
use crate::purchased_products::DbPurchasedProduct;

#[async_trait]
pub trait PurchasedProductsDataProvider: Send + Sync {
    async fn create_purchased_product(
        &self,
        org_id: OrganizationId,
        product_id: ProductId,
        tokens_granted: i64,
        stripe_payment_intent_id: Option<String>,
    ) -> Result<DbPurchasedProduct, DataError>;

    async fn get_purchased_product(
        &self,
        id: PurchasedProductId,
    ) -> Result<Option<DbPurchasedProduct>, DataError>;

    async fn list_by_org(
        &self,
        org_id: OrganizationId,
    ) -> Result<Vec<DbPurchasedProduct>, DataError>;
    async fn list_all(&self) -> Result<Vec<DbPurchasedProduct>, DataError>;
    async fn list_paginated(
        &self,
        org_id: Option<OrganizationId>,
        offset: i64,
        limit: i64,
    ) -> Result<(Vec<DbPurchasedProduct>, i64), DataError>;
    async fn delete_purchased_product(&self, id: PurchasedProductId) -> Result<bool, DataError>;
}
