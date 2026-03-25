use std::sync::Arc;
use async_trait::async_trait;
use crate::domain::db::db_usage::DbUsage;
use crate::domain::service_error::ServiceError;
use crate::usage::data_provider::usage_data_provider::UsageDataProvider;
use crate::usage::usage_provider::UsageProvider;

pub struct UsageProviderService {
    data_provider: Arc<dyn UsageDataProvider>,
}
impl UsageProviderService {
    pub fn new(data_provider: Arc<dyn UsageDataProvider>) -> Self { Self { data_provider } }
}
#[async_trait]
impl UsageProvider for UsageProviderService {
    async fn get_usage(&self, user_id: i32) -> Result<Option<DbUsage>, ServiceError> {
        self.data_provider.get_usage(user_id).await.map_err(ServiceError::from)
    }
    async fn refresh_usage(&self, user_id: i32) -> Result<DbUsage, ServiceError> {
        self.data_provider.refresh_usage(user_id).await.map_err(ServiceError::from)
    }
}
