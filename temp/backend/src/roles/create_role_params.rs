use crate::domain::db::db_organization::OrganizationId;

#[derive(Clone, Debug)]
pub struct CreateRoleParams {
    pub org_id: OrganizationId,
    pub name: String,
    pub is_system: Option<bool>,
}
