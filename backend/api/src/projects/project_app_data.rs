use std::sync::Arc;

use crate::memberships::memberships_provider::MembershipsProvider;

use super::projects_provider::ProjectsProvider;

#[derive(Clone)]
pub struct ProjectAppData {
    pub projects_service: Arc<dyn ProjectsProvider>,
    pub memberships_service: Arc<dyn MembershipsProvider>,
}
