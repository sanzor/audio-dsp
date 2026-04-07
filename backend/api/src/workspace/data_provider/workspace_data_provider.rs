use domain::{db::db_project::ProjectId, tracks::track_subtree::TrackSubtree};

#[async_trait::async_trait]
pub trait WorkspaceDataProvider: Send + Sync {
    async fn get_project_workspace(&self, project_id: ProjectId) -> Result<Vec<TrackSubtree>, String>;
}
