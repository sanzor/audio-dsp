use crate::domain::db::{db_organization::OrganizationId, db_role::RoleId, db_user::UserId};

#[derive(Clone, Debug)]
pub struct CreateMembershipRoleParams {
    pub user_id: UserId,
    pub org_id: OrganizationId,
    pub role_id: RoleId,
}
