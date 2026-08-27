#[derive(Clone, Debug)]
pub struct AcceptInviteParams {
    pub invite_token: String,
    /// user_id of the authenticated caller (from JwtContext)
    pub caller_user_id: domain::domain_user::UserId,
}
