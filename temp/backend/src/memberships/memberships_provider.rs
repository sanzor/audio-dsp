use async_trait::async_trait;

use crate::domain::db::db_organization::OrganizationId;
use crate::domain::db::db_user::UserId;
use crate::domain::service_error::ServiceError;
use crate::memberships::create_membership_params::CreateMembershipParams;
use crate::memberships::DbMembership;

#[async_trait]
pub trait MembershipsProvider: Send + Sync {
    async fn create_membership(
        &self,
        params: CreateMembershipParams,
    ) -> Result<DbMembership, ServiceError>;
    async fn delete_membership(
        &self,
        user_id: UserId,
        org_id: OrganizationId,
    ) -> Result<bool, ServiceError>;
    async fn get_membership(
        &self,
        user_id: UserId,
        org_id: OrganizationId,
    ) -> Result<Option<DbMembership>, ServiceError>;
    async fn list_memberships(
        &self,
        user_id: Option<UserId>,
        org_id: Option<OrganizationId>,
    ) -> Result<Vec<DbMembership>, ServiceError>;
}
