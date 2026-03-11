use async_trait::async_trait;

use crate::{
    domain::{
        data_error::DataError,
        db::{db_organization::OrganizationId, db_role::RoleId},
    },
    roles::DbRole,
};

#[async_trait]
pub trait RolesDataProvider: Send + Sync {
    async fn create_role(
        &self,
        org_id: OrganizationId,
        name: String,
        is_system: bool,
    ) -> Result<DbRole, DataError>;
    async fn update_role(&self, role_id: RoleId, name: String)
        -> Result<Option<DbRole>, DataError>;
    async fn delete_role(&self, role_id: RoleId) -> Result<bool, DataError>;
    async fn get_role(&self, role_id: RoleId) -> Result<Option<DbRole>, DataError>;
    async fn list_roles(&self, org_id: Option<OrganizationId>) -> Result<Vec<DbRole>, DataError>;
}
