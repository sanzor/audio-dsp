use crate::domain::db::db_membership::DbMembership;
use crate::domain::db::db_role::DbRole;
use crate::organizations::DbOrganization;

#[derive(Clone, Debug)]
pub struct CreateOrganizationResult {
    pub organization: DbOrganization,
}

#[derive(Clone, Debug)]
pub struct CreateOrganizationWithOwnerResult {
    pub organization: DbOrganization,
    pub membership: DbMembership,
    pub owner_role: DbRole,
}
