use crate::memberships::DbMembership;

#[derive(Clone, Debug)]
pub struct GetMembershipResult {
    pub membership: DbMembership,
}
