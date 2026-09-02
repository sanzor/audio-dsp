use domain::{db::db_workspace::WorkspaceId, tracks::track_subtree::TrackSubtree};

#[async_trait::async_trait]
pub trait WorkspaceProvider: Send + Sync {
    async fn get_workspace_tracks(
        &self,
        workspace_id: WorkspaceId,
    ) -> Result<Vec<TrackSubtree>, String>;
}
