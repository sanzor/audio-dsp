use crate::domain::db::{db_organization::OrganizationId, db_user::UserId};

#[derive(Clone, Debug)]
pub struct AcceptInviteParams {
    pub user_id: UserId,
    pub org_id: OrganizationId,
    pub invite_token: String,
}
