use domain::db::{DbWorkspace, WorkspaceId};

pub struct CreateWorkspaceParams {
    pub name: String,
    pub created_by: domain::domain_user::UserId,
}

pub struct UpdateWorkspaceParams {
    pub name: String,
}

#[async_trait::async_trait]
pub trait WorkspacesProvider: Send + Sync {
    async fn create_workspace(&self, params: CreateWorkspaceParams) -> Result<DbWorkspace, String>;
    async fn get_workspace(
        &self,
        workspace_id: &WorkspaceId,
    ) -> Result<Option<DbWorkspace>, String>;
    async fn update_workspace(
        &self,
        workspace_id: &WorkspaceId,
        params: UpdateWorkspaceParams,
    ) -> Result<Option<DbWorkspace>, String>;
    async fn delete_workspace(&self, workspace_id: &WorkspaceId) -> Result<bool, String>;
    async fn list_workspaces_for_user(
        &self,
        user_id: domain::domain_user::UserId,
    ) -> Result<Vec<DbWorkspace>, String>;
}
