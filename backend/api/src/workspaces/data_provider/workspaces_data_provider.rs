use domain::db::{DbWorkspace, WorkspaceId};

use crate::workspaces::workspaces_provider::{CreateWorkspaceParams, UpdateWorkspaceParams};

#[async_trait::async_trait]
pub trait WorkspacesDataProvider: Send + Sync {
    async fn create_workspace(&self, params: CreateWorkspaceParams) -> Result<DbWorkspace, String>;
    async fn get_workspace(&self, workspace_id: &WorkspaceId) -> Result<Option<DbWorkspace>, String>;
    async fn update_workspace(
        &self,
        workspace_id: &WorkspaceId,
        params: UpdateWorkspaceParams,
    ) -> Result<Option<DbWorkspace>, String>;
    async fn delete_workspace(&self, workspace_id: &WorkspaceId) -> Result<bool, String>;
    async fn list_workspaces_for_user(&self, user_id: domain::domain_user::UserId) -> Result<Vec<DbWorkspace>, String>;
}
