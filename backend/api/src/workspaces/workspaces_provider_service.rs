use std::sync::Arc;

use domain::db::{DbWorkspace, WorkspaceId};

use super::{
    data_provider::workspaces_data_provider::WorkspacesDataProvider,
    workspaces_provider::{CreateWorkspaceParams, UpdateWorkspaceParams, WorkspacesProvider},
};

pub struct WorkspacesProviderService {
    data_provider: Arc<dyn WorkspacesDataProvider>,
}

impl WorkspacesProviderService {
    pub fn new(data_provider: Arc<dyn WorkspacesDataProvider>) -> Self {
        Self { data_provider }
    }
}

#[async_trait::async_trait]
impl WorkspacesProvider for WorkspacesProviderService {
    async fn create_workspace(&self, params: CreateWorkspaceParams) -> Result<DbWorkspace, String> {
        self.data_provider.create_workspace(params).await
    }

    async fn get_workspace(
        &self,
        workspace_id: &WorkspaceId,
    ) -> Result<Option<DbWorkspace>, String> {
        self.data_provider.get_workspace(workspace_id).await
    }

    async fn update_workspace(
        &self,
        workspace_id: &WorkspaceId,
        params: UpdateWorkspaceParams,
    ) -> Result<Option<DbWorkspace>, String> {
        self.data_provider
            .update_workspace(workspace_id, params)
            .await
    }

    async fn delete_workspace(&self, workspace_id: &WorkspaceId) -> Result<bool, String> {
        self.data_provider.delete_workspace(workspace_id).await
    }

    async fn list_workspaces_for_user(
        &self,
        user_id: domain::domain_user::UserId,
    ) -> Result<Vec<DbWorkspace>, String> {
        self.data_provider.list_workspaces_for_user(user_id).await
    }
}
