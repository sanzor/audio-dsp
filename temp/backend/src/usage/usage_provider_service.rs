use std::sync::Arc;

use async_trait::async_trait;
use tracing::info;

use crate::domain::db::db_organization::OrganizationId;
use crate::domain::db::usage_snapshot::UsageSnapshot;
use crate::domain::service_error::ServiceError;
use crate::usage::data_provider::usage_data_provider::UsageDataProvider;
use crate::usage::usage_provider::UsageProvider;

pub struct UsageProviderService {
    data_provider: Arc<dyn UsageDataProvider>,
}

impl UsageProviderService {
    pub fn new(data_provider: Arc<dyn UsageDataProvider>) -> Self {
        Self { data_provider }
    }
}

#[async_trait]
impl UsageProvider for UsageProviderService {
    async fn get_usage(
        &self,
        org_id: OrganizationId,
    ) -> Result<Option<UsageSnapshot>, ServiceError> {
        info!(org_id, "get usage requested");
        self.data_provider
            .get_usage(org_id)
            .await
            .map_err(ServiceError::from)
    }
}
