use std::sync::Arc;

use crate::organizations::organization_provider::OrganizationProvider;

#[derive(Clone)]
pub struct OrganizationsAppData {
    pub organization_provider: Arc<dyn OrganizationProvider>,
}
