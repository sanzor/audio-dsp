use async_trait::async_trait;

use crate::{
    domain::{
        data_error::DataError,
        db::{db_organization::OrganizationId, db_role::RoleId, db_user::UserId},
    },
    membership_roles::DbMembershipRole,
};

#[async_trait]
pub trait MembershipRolesDataProvider: Send + Sync {
    async fn create_membership_role(
        &self,
        user_id: UserId,
        org_id: OrganizationId,
        role_id: RoleId,
    ) -> Result<DbMembershipRole, DataError>;
    async fn delete_membership_role(
        &self,
        user_id: UserId,
        org_id: OrganizationId,
        role_id: RoleId,
    ) -> Result<bool, DataError>;
    async fn get_membership_role(
        &self,
        user_id: UserId,
        org_id: OrganizationId,
        role_id: RoleId,
    ) -> Result<Option<DbMembershipRole>, DataError>;
    async fn list_membership_roles(
        &self,
        user_id: Option<UserId>,
        org_id: Option<OrganizationId>,
        role_id: Option<RoleId>,
    ) -> Result<Vec<DbMembershipRole>, DataError>;
}
