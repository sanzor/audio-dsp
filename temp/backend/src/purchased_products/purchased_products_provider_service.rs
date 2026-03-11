use std::sync::Arc;

use async_trait::async_trait;
use tracing::info;

use crate::domain::db::db_organization::OrganizationId;
use crate::domain::db::db_purchased_product::PurchasedProductId;
use crate::domain::service_error::ServiceError;
use crate::purchased_products::data_provider::purchased_products_data_provider::PurchasedProductsDataProvider;
use crate::purchased_products::purchased_products_provider::PurchasedProductsProvider;
use crate::purchased_products::DbPurchasedProduct;

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
    async fn get_purchased_product(
        &self,
        id: PurchasedProductId,
    ) -> Result<Option<DbPurchasedProduct>, ServiceError> {
        info!(id, "get purchased product requested");
        self.data_provider
            .get_purchased_product(id)
            .await
            .map_err(ServiceError::from)
    }

    async fn list_by_org(
        &self,
        org_id: OrganizationId,
    ) -> Result<Vec<DbPurchasedProduct>, ServiceError> {
        info!(org_id, "list purchased products by org requested");
        self.data_provider
            .list_by_org(org_id)
            .await
            .map_err(ServiceError::from)
    }

    async fn list_all(&self) -> Result<Vec<DbPurchasedProduct>, ServiceError> {
        info!("list all purchased products requested");
        self.data_provider
            .list_all()
            .await
            .map_err(ServiceError::from)
    }

    async fn list_paginated(
        &self,
        org_id: Option<OrganizationId>,
        offset: i64,
        limit: i64,
    ) -> Result<(Vec<DbPurchasedProduct>, i64), ServiceError> {
        info!(
            org_id,
            offset, limit, "list paginated purchased products requested"
        );
        self.data_provider
            .list_paginated(org_id, offset, limit)
            .await
            .map_err(ServiceError::from)
    }
}
