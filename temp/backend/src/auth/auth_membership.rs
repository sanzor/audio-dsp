use crate::domain::db::{db_organization::OrganizationId, db_user::UserId};

#[derive(Clone, Debug)]
pub struct AuthMembership {
    pub user_id: UserId,
    pub org_id: OrganizationId,
    pub token: String, // <--- Return a Scoped Token here!
    pub roles: Vec<String>,
}
