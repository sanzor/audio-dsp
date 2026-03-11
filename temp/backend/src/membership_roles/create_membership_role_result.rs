use crate::membership_roles::DbMembershipRole;

#[derive(Clone, Debug)]
pub struct CreateMembershipRoleResult {
    pub membership_role: DbMembershipRole,
}
