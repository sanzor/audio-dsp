use std::sync::Arc;

use domain::db::{DbProject, ProjectId};

use super::{
    data_provider::projects_data_provider::ProjectsDataProvider,
    projects_provider::{CreateProjectParams, ProjectsProvider, UpdateProjectParams},
};

pub struct ProjectsProviderService {
    data_provider: Arc<dyn ProjectsDataProvider>,
}

impl ProjectsProviderService {
    pub fn new(data_provider: Arc<dyn ProjectsDataProvider>) -> Self {
        Self { data_provider }
    }
}

#[async_trait::async_trait]
impl ProjectsProvider for ProjectsProviderService {
    async fn create_project(&self, params: CreateProjectParams) -> Result<DbProject, String> {
        self.data_provider.create_project(params).await
    }

    async fn get_project(&self, project_id: &ProjectId) -> Result<Option<DbProject>, String> {
        self.data_provider.get_project(project_id).await
    }

    async fn update_project(
        &self,
        project_id: &ProjectId,
        params: UpdateProjectParams,
    ) -> Result<Option<DbProject>, String> {
        self.data_provider.update_project(project_id, params).await
    }

    async fn delete_project(&self, project_id: &ProjectId) -> Result<bool, String> {
        self.data_provider.delete_project(project_id).await
    }

    async fn list_projects_for_user(&self, user_id: i32) -> Result<Vec<DbProject>, String> {
        self.data_provider.list_projects_for_user(user_id).await
    }
}
