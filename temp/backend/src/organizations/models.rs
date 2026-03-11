#[derive(Clone, Debug)]
pub struct CreateOrganizationResult {
    pub organization: crate::domain::db::db_organization::DbOrganization,
}

#[derive(Clone, Debug)]
pub struct UpdateOrganizationResult {
    pub organization: crate::domain::db::db_organization::DbOrganization,
}

#[derive(Clone, Debug)]
pub struct GetOrganizationResult {
    pub organization: crate::domain::db::db_organization::DbOrganization,
}

#[derive(Clone, Debug)]
pub struct ListOrganizationsResult {
    pub organizations: Vec<crate::domain::db::db_organization::DbOrganization>,
}
