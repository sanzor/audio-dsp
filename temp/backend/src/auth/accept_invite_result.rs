use crate::auth::auth_membership::AuthMembership;

#[derive(Clone, Debug)]
pub struct AcceptInviteResult {
    pub membership: AuthMembership,
}
