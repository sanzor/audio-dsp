use crate::services::organizations::organization::Organization;

#[derive(Clone, Debug)]
pub struct GetOrganizationResult {
    pub organization: Organization,
}
