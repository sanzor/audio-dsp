use crate::organizations::DbOrganization;

#[derive(Clone, Debug)]
pub struct UpdateOrganizationResult {
    pub organization: DbOrganization,
}
