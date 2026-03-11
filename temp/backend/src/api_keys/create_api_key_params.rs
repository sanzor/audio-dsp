use crate::domain::db::{db_organization::OrganizationId, db_user::UserId};

#[derive(Clone, Debug)]
pub struct CreateApiKeyParams {
    pub org_id: OrganizationId,
    pub label: Option<String>,
    pub created_by: Option<UserId>,
}
