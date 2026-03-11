use async_trait::async_trait;

use crate::domain::data_error::DataError;
use crate::domain::db::db_organization::{DbOrganization, OrganizationId};
use crate::domain::db::db_user::UserId;
use crate::organizations::create_organization_params::CreateOrganizationParams;
use crate::organizations::create_organization_result::CreateOrganizationWithOwnerResult;
use crate::organizations::update_organization_params::UpdateOrganizationParams;

#[async_trait]
pub trait OrganizationDataProvider: Send + Sync {
    async fn create_organization(
        &self,
        params: CreateOrganizationParams,
    ) -> Result<DbOrganization, DataError>;
    async fn create_organization_with_owner(
        &self,
        params: CreateOrganizationParams,
        user_id: UserId,
    ) -> Result<CreateOrganizationWithOwnerResult, DataError>;
    async fn update_organization(
        &self,
        org_id: OrganizationId,
        params: UpdateOrganizationParams,
    ) -> Result<Option<DbOrganization>, DataError>;
    async fn delete_organization(&self, org_id: OrganizationId) -> Result<bool, DataError>;
    async fn get_organization(
        &self,
        org_id: OrganizationId,
    ) -> Result<Option<DbOrganization>, DataError>;
    async fn get_all_organizations(&self) -> Result<Vec<DbOrganization>, DataError>;
}
