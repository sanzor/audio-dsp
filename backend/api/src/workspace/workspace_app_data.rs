use std::sync::Arc;

use super::workspace_provider::WorkspaceProvider;

#[derive(Clone)]
pub struct WorkspaceAppData {
    pub workspace_service: Arc<dyn WorkspaceProvider>,
}
