use async_trait::async_trait;

use crate::domain::db::db_organization::OrganizationId;
use crate::domain::db::db_purchased_product::PurchasedProductId;
use crate::domain::service_error::ServiceError;
use crate::purchased_products::create_purchased_product_params::CreatePurchasedProductParams;
use crate::purchased_products::DbPurchasedProduct;

#[async_trait]
pub trait PurchasedProductsProvider: Send + Sync {
    async fn get_purchased_product(
        &self,
        id: PurchasedProductId,
    ) -> Result<Option<DbPurchasedProduct>, ServiceError>;

    async fn list_by_org(
        &self,
        org_id: OrganizationId,
    ) -> Result<Vec<DbPurchasedProduct>, ServiceError>;

    async fn list_all(&self) -> Result<Vec<DbPurchasedProduct>, ServiceError>;

    async fn list_paginated(
        &self,
        org_id: Option<OrganizationId>,
        offset: i64,
        limit: i64,
    ) -> Result<(Vec<DbPurchasedProduct>, i64), ServiceError>;
}
