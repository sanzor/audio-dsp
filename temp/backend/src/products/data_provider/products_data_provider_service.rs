use async_trait::async_trait;
use sqlx::PgPool;
use tracing::error;

use crate::domain::data_error::DataError;
use crate::domain::db::db_product::ProductId;
use crate::products::data_provider::products_data_provider::ProductsDataProvider;
use crate::products::DbProduct;

pub struct ProductsDataProviderService {
    pool: PgPool,
}

impl ProductsDataProviderService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ProductsDataProvider for ProductsDataProviderService {
    async fn create_product(
        &self,
        name: String,
        description: Option<String>,
        token_amount: i64,
        price_cents: i64,
        currency: String,
        stripe_price_id: Option<String>,
        stripe_product_id: Option<String>,
    ) -> Result<DbProduct, DataError> {
        let record = sqlx::query_as::<_, DbProduct>(
            "INSERT INTO products (name, description, token_amount, price_cents, currency, stripe_price_id, stripe_product_id) \
             VALUES ($1, $2, $3, $4, $5, $6, $7) \
             RETURNING id, name, description, token_amount, price_cents, currency, stripe_price_id, \
                       stripe_product_id, is_active, created_at::text, updated_at::text",
        )
        .bind(name)
        .bind(description)
        .bind(token_amount)
        .bind(price_cents)
        .bind(currency)
        .bind(stripe_price_id)
        .bind(stripe_product_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            error!(error = %e, "create product failed");
            DataError::from(e)
        })?;

        Ok(record)
    }

    async fn update_product(
        &self,
        id: ProductId,
        name: Option<String>,
        description: Option<String>,
        token_amount: Option<i64>,
        price_cents: Option<i64>,
        stripe_price_id: Option<String>,
        is_active: Option<bool>,
    ) -> Result<Option<DbProduct>, DataError> {
        let record = sqlx::query_as::<_, DbProduct>(
            "UPDATE products \
             SET name            = COALESCE($2, name), \
                 description     = COALESCE($3, description), \
                 token_amount    = COALESCE($4, token_amount), \
                 price_cents     = COALESCE($5, price_cents), \
                 stripe_price_id = COALESCE($6, stripe_price_id), \
                 is_active       = COALESCE($7, is_active), \
                 updated_at      = now() \
             WHERE id = $1 \
             RETURNING id, name, description, token_amount, price_cents, currency, stripe_price_id, \
                       stripe_product_id, is_active, created_at::text, updated_at::text",
        )
        .bind(id)
        .bind(name)
        .bind(description)
        .bind(token_amount)
        .bind(price_cents)
        .bind(stripe_price_id)
        .bind(is_active)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            error!(id, error = %e, "update product failed");
            DataError::from(e)
        })?;

        Ok(record)
    }

    async fn delete_product(&self, id: ProductId) -> Result<bool, DataError> {
        let result = sqlx::query("DELETE FROM products WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| {
                error!(id, error = %e, "delete product failed");
                DataError::from(e)
            })?;

        Ok(result.rows_affected() > 0)
    }

    async fn get_product(&self, id: ProductId) -> Result<Option<DbProduct>, DataError> {
        let record = sqlx::query_as::<_, DbProduct>(
            "SELECT id, name, description, token_amount, price_cents, currency, stripe_price_id, \
                    stripe_product_id, is_active, created_at::text, updated_at::text \
             FROM products WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            error!(id, error = %e, "get product failed");
            DataError::from(e)
        })?;

        Ok(record)
    }

    async fn list_products(&self) -> Result<Vec<DbProduct>, DataError> {
        let records = sqlx::query_as::<_, DbProduct>(
            "SELECT id, name, description, token_amount, price_cents, currency, stripe_price_id, \
                    stripe_product_id, is_active, created_at::text, updated_at::text \
             FROM products ORDER BY id",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            error!(error = %e, "list products failed");
            DataError::from(e)
        });
        records
    }
}
