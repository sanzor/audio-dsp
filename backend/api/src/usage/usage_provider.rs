use async_trait::async_trait;
use crate::domain::db::db_usage::DbUsage;
use crate::domain::service_error::ServiceError;

#[async_trait]
pub trait UsageProvider: Send + Sync {
    async fn get_usage(&self, user_id: i64) -> Result<Option<DbUsage>, ServiceError>;
    async fn refresh_usage(&self, user_id: i64) -> Result<DbUsage, ServiceError>;
}
