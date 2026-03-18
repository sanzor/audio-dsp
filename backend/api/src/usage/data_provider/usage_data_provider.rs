use async_trait::async_trait;
use crate::domain::data_error::DataError;
use crate::domain::db::db_usage::DbUsage;

#[async_trait]
pub trait UsageDataProvider: Send + Sync {
    async fn get_usage(&self, user_id: &str) -> Result<Option<DbUsage>, DataError>;
    async fn refresh_usage(&self, user_id: &str) -> Result<DbUsage, DataError>;
}
