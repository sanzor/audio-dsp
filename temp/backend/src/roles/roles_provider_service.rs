use std::sync::Arc;

use async_trait::async_trait;
use tracing::info;

use crate::domain::db::db_organization::OrganizationId;
use crate::domain::db::db_role::RoleId;
use crate::domain::service_error::ServiceError;
use crate::roles::create_role_params::CreateRoleParams;
use crate::roles::data_provider::roles_data_provider::RolesDataProvider;
use crate::roles::update_role_params::UpdateRoleParams;
use crate::roles::{roles_provider::RolesProvider, DbRole};

pub struct RolesProviderService {
    data_provider: Arc<dyn RolesDataProvider>,
}

impl RolesProviderService {
    pub fn new(data_provider: Arc<dyn RolesDataProvider>) -> Self {
        Self { data_provider }
    }
}

#[async_trait]
impl RolesProvider for RolesProviderService {
    async fn create_role(&self, params: CreateRoleParams) -> Result<DbRole, ServiceError> {
        info!(org_id = params.org_id, name = %params.name, "create role requested");
        Ok(self
            .data_provider
            .create_role(
                params.org_id,
                params.name,
                params.is_system.unwrap_or(false),
            )
            .await?)
    }

    async fn update_role(
        &self,
        role_id: RoleId,
        params: UpdateRoleParams,
    ) -> Result<Option<DbRole>, ServiceError> {
        info!(role_id, "update role requested");
        Ok(self.data_provider.update_role(role_id, params.name).await?)
    }

    async fn delete_role(&self, role_id: RoleId) -> Result<bool, ServiceError> {
        info!(role_id, "delete role requested");
        Ok(self.data_provider.delete_role(role_id).await?)
    }

    async fn get_role(&self, role_id: RoleId) -> Result<Option<DbRole>, ServiceError> {
        info!(role_id, "get role requested");
        Ok(self.data_provider.get_role(role_id).await?)
    }

    async fn list_roles(
        &self,
        org_id: Option<OrganizationId>,
    ) -> Result<Vec<DbRole>, ServiceError> {
        Ok(self.data_provider.list_roles(org_id).await?)
    }
}
