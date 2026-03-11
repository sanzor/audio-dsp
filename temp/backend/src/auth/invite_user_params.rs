use crate::domain::db::db_organization::OrganizationId;

pub struct InviteUserParams {
    pub email: String,
    pub org_id: OrganizationId,
}
