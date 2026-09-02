use crate::domain::data_error::DataError;
use crate::domain::db::db_purchased_product::{DbPurchasedProduct, PurchasedProductId};
use crate::purchased_products::create_purchased_product_params::CreatePurchasedProductParams;
use crate::purchased_products::data_provider::purchased_products_data_provider::PurchasedProductsDataProvider;
use async_trait::async_trait;
use sqlx::PgPool;
use tracing::error;

const SELECT: &str = "SELECT id, user_id, product_id, purchased_at::text FROM purchased_products";

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
        params: CreatePurchasedProductParams,
    ) -> Result<DbPurchasedProduct, DataError> {
        sqlx::query_as::<_, DbPurchasedProduct>(
            "INSERT INTO purchased_products (user_id, product_id) VALUES ($1, $2) \
             RETURNING id, user_id, product_id, purchased_at::text",
        )
        .bind(params.user_id)
        .bind(params.product_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            error!(error = %e, "create purchased product failed");
            DataError::from(e)
        })
    }

    async fn get_purchased_product(
        &self,
        id: PurchasedProductId,
    ) -> Result<Option<DbPurchasedProduct>, DataError> {
        sqlx::query_as::<_, DbPurchasedProduct>(&format!("{SELECT} WHERE id = $1"))
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| {
                error!(id, error = %e, "get purchased product failed");
                DataError::from(e)
            })
    }

    async fn list_by_user(
        &self,
        user_id: domain::domain_user::UserId,
    ) -> Result<Vec<DbPurchasedProduct>, DataError> {
        sqlx::query_as::<_, DbPurchasedProduct>(&format!(
            "{SELECT} WHERE user_id = $1 ORDER BY purchased_at DESC"
        ))
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            error!(error = %e, "list purchased products failed");
            DataError::from(e)
        })
    }

    async fn delete_purchased_product(&self, id: PurchasedProductId) -> Result<bool, DataError> {
        sqlx::query("DELETE FROM purchased_products WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map(|r| r.rows_affected() > 0)
            .map_err(|e| {
                error!(id, error = %e, "delete purchased product failed");
                DataError::from(e)
            })
    }
}
