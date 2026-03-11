use async_trait::async_trait;

use crate::domain::db::{db_organization::OrganizationId, db_role::RoleId, db_user::UserId};
use crate::domain::service_error::ServiceError;
use crate::membership_roles::{
    create_membership_role_params::CreateMembershipRoleParams, DbMembershipRole,
};

#[async_trait]
pub trait MembershipRolesProvider: Send + Sync {
    async fn create_membership_role(
        &self,
        params: CreateMembershipRoleParams,
    ) -> Result<DbMembershipRole, ServiceError>;
    async fn delete_membership_role(
        &self,
        user_id: UserId,
        org_id: OrganizationId,
        role_id: RoleId,
    ) -> Result<bool, ServiceError>;
    async fn get_membership_role(
        &self,
        user_id: UserId,
        org_id: OrganizationId,
        role_id: RoleId,
    ) -> Result<Option<DbMembershipRole>, ServiceError>;
    async fn list_membership_roles(
        &self,
        user_id: Option<UserId>,
        org_id: Option<OrganizationId>,
        role_id: Option<RoleId>,
    ) -> Result<Vec<DbMembershipRole>, ServiceError>;
}
