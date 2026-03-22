use std::sync::Arc;

use crate::{
    memberships::memberships_provider::MembershipsProvider,
    projects::projects_provider::ProjectsProvider,
};

#[derive(Clone)]
pub struct AppData {
    pub projects_service: Arc<dyn ProjectsProvider>,
    pub memberships_service: Arc<dyn MembershipsProvider>,
}
