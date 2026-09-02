use crate::domain::data_error::DataError;
use crate::domain::db::db_product::{DbProduct, ProductId};
use crate::products::create_product_params::CreateProductParams;
use crate::products::data_provider::products_data_provider::ProductsDataProvider;
use crate::products::update_product_params::UpdateProductParams;
use async_trait::async_trait;
use sqlx::PgPool;
use tracing::error;

const SELECT: &str = "SELECT id, name, description, tier, price_cents, currency, is_active, created_at::text FROM products";

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
    async fn create_product(&self, params: CreateProductParams) -> Result<DbProduct, DataError> {
        sqlx::query_as::<_, DbProduct>(
            "INSERT INTO products (name, description, tier, price_cents, currency, is_active) \
             VALUES ($1, $2, $3, $4, $5, true) \
             RETURNING id, name, description, tier, price_cents, currency, is_active, created_at::text"
        )
        .bind(&params.name)
        .bind(&params.description)
        .bind(&params.tier)
        .bind(params.price_cents)
        .bind(&params.currency)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| { error!(error = %e, "create product failed"); DataError::from(e) })
    }

    async fn get_product(&self, id: ProductId) -> Result<Option<DbProduct>, DataError> {
        sqlx::query_as::<_, DbProduct>(&format!("{SELECT} WHERE id = $1"))
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| {
                error!(id, error = %e, "get product failed");
                DataError::from(e)
            })
    }

    async fn update_product(
        &self,
        id: ProductId,
        params: UpdateProductParams,
    ) -> Result<Option<DbProduct>, DataError> {
        let existing = self.get_product(id).await?;
        if existing.is_none() {
            return Ok(None);
        }
        let existing = existing.unwrap();
        let name = params.name.unwrap_or(existing.name);
        let description = params.description.unwrap_or(existing.description);
        let tier = params.tier.unwrap_or(existing.tier);
        let price_cents = params.price_cents.unwrap_or(existing.price_cents);
        let currency = params.currency.unwrap_or(existing.currency);
        let is_active = params.is_active.unwrap_or(existing.is_active);
        sqlx::query_as::<_, DbProduct>(
            "UPDATE products SET name=$1, description=$2, tier=$3, price_cents=$4, currency=$5, is_active=$6 \
             WHERE id=$7 RETURNING id, name, description, tier, price_cents, currency, is_active, created_at::text"
        )
        .bind(&name).bind(&description).bind(&tier).bind(price_cents).bind(&currency).bind(is_active).bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| { error!(id, error = %e, "update product failed"); DataError::from(e) })
    }

    async fn delete_product(&self, id: ProductId) -> Result<bool, DataError> {
        sqlx::query("DELETE FROM products WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map(|r| r.rows_affected() > 0)
            .map_err(|e| {
                error!(id, error = %e, "delete product failed");
                DataError::from(e)
            })
    }

    async fn list_products(&self, active_only: bool) -> Result<Vec<DbProduct>, DataError> {
        let query = if active_only {
            format!("{SELECT} WHERE is_active = true ORDER BY id")
        } else {
            format!("{SELECT} ORDER BY id")
        };
        sqlx::query_as::<_, DbProduct>(&query)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| {
                error!(error = %e, "list products failed");
                DataError::from(e)
            })
    }
}
