use async_trait::async_trait;

use crate::domain::db::db_organization::OrganizationId;
use crate::domain::db::db_user::UserId;
use crate::domain::service_error::ServiceError;
use crate::organizations::create_organization_params::CreateOrganizationParams;
use crate::organizations::create_organization_result::CreateOrganizationWithOwnerResult;
use crate::organizations::update_organization_params::UpdateOrganizationParams;
use crate::organizations::{
    CreateOrganizationResult, DbOrganization, GetOrganizationResult, UpdateOrganizationResult,
};

#[async_trait]
pub trait OrganizationProvider: Send + Sync {
    async fn create_organization(
        &self,
        params: CreateOrganizationParams,
    ) -> Result<CreateOrganizationResult, ServiceError>;
    async fn create_organization_with_owner(
        &self,
        params: CreateOrganizationParams,
        user_id: UserId,
    ) -> Result<CreateOrganizationWithOwnerResult, ServiceError>;
    async fn update_organization(
        &self,
        org_id: OrganizationId,
        params: UpdateOrganizationParams,
    ) -> Result<Option<UpdateOrganizationResult>, ServiceError>;
    async fn delete_organization(&self, org_id: OrganizationId) -> Result<bool, ServiceError>;
    async fn get_organization(
        &self,
        org_id: OrganizationId,
    ) -> Result<Option<GetOrganizationResult>, ServiceError>;
    async fn get_all_organizations(&self) -> Result<Vec<DbOrganization>, ServiceError>;
}
