use std::sync::Arc;

use crate::{
    memberships::memberships_provider::MembershipsProvider,
    workspaces::workspaces_provider::WorkspacesProvider,
};

#[derive(Clone)]
pub struct AppData {
    pub workspaces_service: Arc<dyn WorkspacesProvider>,
    pub memberships_service: Arc<dyn MembershipsProvider>,
}
