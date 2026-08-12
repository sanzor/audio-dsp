use std::sync::Arc;

use crate::memberships::memberships_provider::MembershipsProvider;

use super::workspaces_provider::WorkspacesProvider;

#[derive(Clone)]
pub struct WorkspacesAppData {
    pub workspaces_service: Arc<dyn WorkspacesProvider>,
    pub memberships_service: Arc<dyn MembershipsProvider>,
}
