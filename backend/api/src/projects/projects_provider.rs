use domain::db::{DbProject, ProjectId};

pub struct CreateProjectParams {
    pub name: String,
    pub created_by: String,
}

pub struct UpdateProjectParams {
    pub name: String,
}

#[async_trait::async_trait]
pub trait ProjectsProvider: Send + Sync {
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
