use async_trait::async_trait;

use crate::domain::db::db_organization::OrganizationId;
use crate::domain::db::db_role::RoleId;
use crate::domain::service_error::ServiceError;
use crate::roles::{
    create_role_params::CreateRoleParams, update_role_params::UpdateRoleParams, DbRole,
};

#[async_trait]
pub trait RolesProvider: Send + Sync {
    async fn create_role(&self, params: CreateRoleParams) -> Result<DbRole, ServiceError>;
    async fn update_role(
        &self,
        role_id: RoleId,
        params: UpdateRoleParams,
    ) -> Result<Option<DbRole>, ServiceError>;
    async fn delete_role(&self, role_id: RoleId) -> Result<bool, ServiceError>;
    async fn get_role(&self, role_id: RoleId) -> Result<Option<DbRole>, ServiceError>;
    async fn list_roles(&self, org_id: Option<OrganizationId>)
        -> Result<Vec<DbRole>, ServiceError>;
}
