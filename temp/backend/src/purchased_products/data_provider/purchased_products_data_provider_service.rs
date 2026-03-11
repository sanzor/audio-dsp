use async_trait::async_trait;
use sqlx::PgPool;
use tracing::error;

use crate::domain::data_error::DataError;
use crate::domain::db::db_organization::OrganizationId;
use crate::domain::db::db_product::ProductId;
use crate::domain::db::db_purchased_product::PurchasedProductId;
use crate::purchased_products::data_provider::purchased_products_data_provider::PurchasedProductsDataProvider;
use crate::purchased_products::DbPurchasedProduct;

pub struct PurchasedProductsDataProviderService {
    pool: PgPool,
}

impl PurchasedProductsDataProviderService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PurchasedProductsDataProvider for PurchasedProductsDataProviderService {
    async fn create_purchased_product(
        &self,
        org_id: OrganizationId,
        product_id: ProductId,
        tokens_granted: i64,
        stripe_payment_intent_id: Option<String>,
    ) -> Result<DbPurchasedProduct, DataError> {
        let record = sqlx::query_as::<_, DbPurchasedProduct>(
            "INSERT INTO purchased_products (org_id, product_id, tokens_granted, stripe_payment_intent_id) \
             VALUES ($1, $2, $3, $4) \
             RETURNING id, org_id, product_id, tokens_granted, stripe_payment_intent_id, purchased_at::text",
        )
        .bind(org_id)
        .bind(product_id)
        .bind(tokens_granted)
        .bind(stripe_payment_intent_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            error!(org_id, product_id, error = %e, "create purchased product failed");
            DataError::from(e)
        })?;

        Ok(record)
    }

    async fn get_purchased_product(
        &self,
        id: PurchasedProductId,
    ) -> Result<Option<DbPurchasedProduct>, DataError> {
        let record = sqlx::query_as::<_, DbPurchasedProduct>(
            "SELECT id, org_id, product_id, tokens_granted, stripe_payment_intent_id, purchased_at::text \
             FROM purchased_products WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            error!(id, error = %e, "get purchased product failed");
            DataError::from(e)
        })?;

        Ok(record)
    }

    async fn list_by_org(
        &self,
        org_id: OrganizationId,
    ) -> Result<Vec<DbPurchasedProduct>, DataError> {
        let records = sqlx::query_as::<_, DbPurchasedProduct>(
            "SELECT id, org_id, product_id, tokens_granted, stripe_payment_intent_id, purchased_at::text \
             FROM purchased_products WHERE org_id = $1 ORDER BY purchased_at DESC",
        )
        .bind(org_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            error!(org_id, error = %e, "list purchased products by org failed");
            DataError::from(e)
        })?;

        Ok(records)
    }

    async fn list_all(&self) -> Result<Vec<DbPurchasedProduct>, DataError> {
        let records = sqlx::query_as::<_, DbPurchasedProduct>(
            "SELECT id, org_id, product_id, tokens_granted, stripe_payment_intent_id, purchased_at::text \
             FROM purchased_products ORDER BY purchased_at DESC",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            error!(error = %e, "list all purchased products failed");
            DataError::from(e)
        })?;

        Ok(records)
    }

    async fn list_paginated(
        &self,
        org_id: Option<OrganizationId>,
        offset: i64,
        limit: i64,
    ) -> Result<(Vec<DbPurchasedProduct>, i64), DataError> {
        let total: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM purchased_products \
             WHERE ($1::bigint IS NULL OR org_id = $1::bigint)",
        )
        .bind(org_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            error!(error = %e, "list paginated purchased products count failed");
            DataError::from(e)
        })?;

        let records = sqlx::query_as::<_, DbPurchasedProduct>(
            "SELECT id, org_id, product_id, tokens_granted, stripe_payment_intent_id, purchased_at::text \
             FROM purchased_products \
             WHERE ($1::bigint IS NULL OR org_id = $1::bigint) \
             ORDER BY purchased_at DESC \
             LIMIT $2 OFFSET $3",
        )
        .bind(org_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            error!(error = %e, "list paginated purchased products failed");
            DataError::from(e)
        })?;

        Ok((records, total))
    }

    async fn delete_purchased_product(&self, id: PurchasedProductId) -> Result<bool, DataError> {
        let result = sqlx::query("DELETE FROM purchased_products WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| {
                error!(id, error = %e, "delete purchased product failed");
                DataError::from(e)
            })?;

        Ok(result.rows_affected() > 0)
    }
}
