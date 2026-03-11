use std::sync::Arc;

use async_trait::async_trait;
use tracing::info;

use crate::access_control::access_control_provider::AccessControlProvider;
use crate::access_control::data_provider::access_control_data_provider::AccessControlDataProvider;
use crate::domain::db::db_organization::OrganizationId;
use crate::domain::db::db_user::UserId;
use crate::domain::service_error::ServiceError;

pub struct AccessControlProviderService {
    data_provider: Arc<dyn AccessControlDataProvider>,
}

impl AccessControlProviderService {
    pub fn new(data_provider: Arc<dyn AccessControlDataProvider>) -> Self {
        Self { data_provider }
    }
}

#[async_trait]
impl AccessControlProvider for AccessControlProviderService {
    async fn has_permission(
        &self,
        user_id: UserId,
        org_id: OrganizationId,
        permission_key: &str,
    ) -> Result<bool, ServiceError> {
        info!(
            user_id,
            org_id, permission_key, "check permission requested"
        );
        Ok(self
            .data_provider
            .has_permission(user_id, org_id, permission_key)
            .await?)
    }

    async fn permissions_for_user(
        &self,
        user_id: UserId,
        org_id: OrganizationId,
    ) -> Result<Vec<String>, ServiceError> {
        info!(user_id, org_id, "list permissions for user requested");
        Ok(self
            .data_provider
            .permissions_for_user(user_id, org_id)
            .await?)
    }
}
