use crate::membership_roles::DbMembershipRole;

#[derive(Clone, Debug)]
pub struct GetMembershipRoleResult {
    pub membership_role: DbMembershipRole,
}
