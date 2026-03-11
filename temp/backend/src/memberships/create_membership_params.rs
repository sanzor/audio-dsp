use crate::domain::db::{db_organization::OrganizationId, db_user::UserId};

#[derive(Clone, Debug)]
pub struct CreateMembershipParams {
    pub user_id: UserId,
    pub org_id: OrganizationId,
}
