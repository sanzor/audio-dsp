use std::sync::Arc;

use domain::{db::db_workspace::WorkspaceId, tracks::track_subtree::TrackSubtree};

use super::{
    data_provider::workspace_data_provider::WorkspaceDataProvider,
    workspace_provider::WorkspaceProvider,
};

pub struct WorkspaceProviderService {
    data_provider: Arc<dyn WorkspaceDataProvider>,
}

impl WorkspaceProviderService {
    pub fn new(data_provider: Arc<dyn WorkspaceDataProvider>) -> Self {
        Self { data_provider }
    }
}

#[async_trait::async_trait]
impl WorkspaceProvider for WorkspaceProviderService {
    async fn get_workspace_tracks(&self, workspace_id: WorkspaceId) -> Result<Vec<TrackSubtree>, String> {
        self.data_provider.get_workspace_tracks(workspace_id).await
    }
}
