use crate::memberships::DbMembership;

#[derive(Clone, Debug)]
pub struct CreateMembershipResult {
    pub membership: DbMembership,
}
