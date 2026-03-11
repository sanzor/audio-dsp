use async_trait::async_trait;

use crate::domain::data_error::DataError;
use crate::domain::db::db_product::ProductId;
use crate::products::DbProduct;

#[async_trait]
pub trait ProductsDataProvider: Send + Sync {
    async fn create_product(
        &self,
        name: String,
        description: Option<String>,
        token_amount: i64,
        price_cents: i64,
        currency: String,
        stripe_price_id: Option<String>,
        stripe_product_id: Option<String>,
    ) -> Result<DbProduct, DataError>;

    async fn update_product(
        &self,
        id: ProductId,
        name: Option<String>,
        description: Option<String>,
        token_amount: Option<i64>,
        price_cents: Option<i64>,
        stripe_price_id: Option<String>,
        is_active: Option<bool>,
    ) -> Result<Option<DbProduct>, DataError>;

    async fn delete_product(&self, id: ProductId) -> Result<bool, DataError>;
    async fn get_product(&self, id: ProductId) -> Result<Option<DbProduct>, DataError>;
    async fn list_products(&self) -> Result<Vec<DbProduct>, DataError>;
}
