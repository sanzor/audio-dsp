use domain::db::{DbProject, ProjectId};

use crate::projects::projects_provider::{CreateProjectParams, UpdateProjectParams};

#[async_trait::async_trait]
pub trait ProjectsDataProvider: Send + Sync {
    async fn create_project(&self, params: CreateProjectParams) -> Result<DbProject, String>;
    async fn get_project(&self, project_id: &ProjectId) -> Result<Option<DbProject>, String>;
    async fn update_project(
        &self,
        project_id: &ProjectId,
        params: UpdateProjectParams,
    ) -> Result<Option<DbProject>, String>;
    async fn delete_project(&self, project_id: &ProjectId) -> Result<bool, String>;
    async fn list_projects_for_user(&self, user_id: &str) -> Result<Vec<DbProject>, String>;
}
