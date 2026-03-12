use domain::db::{DbProject, ProjectId};
use sqlx::PgPool;
use ulid::Ulid;

use crate::projects::projects_provider::{CreateProjectParams, UpdateProjectParams};

use super::projects_data_provider::ProjectsDataProvider;

pub struct PostgresProjectsDataProvider {
    pool: PgPool,
}

impl PostgresProjectsDataProvider {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl ProjectsDataProvider for PostgresProjectsDataProvider {
    async fn create_project(&self, params: CreateProjectParams) -> Result<DbProject, String> {
        let id = Ulid::new().to_string();
        sqlx::query_as::<_, DbProject>(
            "INSERT INTO projects (project_id, name, created_by) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(&id)
        .bind(&params.name)
        .bind(&params.created_by)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn get_project(&self, project_id: &ProjectId) -> Result<Option<DbProject>, String> {
        sqlx::query_as::<_, DbProject>("SELECT * FROM projects WHERE project_id = $1")
            .bind(project_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| e.to_string())
    }

    async fn update_project(
        &self,
        project_id: &ProjectId,
        params: UpdateProjectParams,
    ) -> Result<Option<DbProject>, String> {
        sqlx::query_as::<_, DbProject>(
            "UPDATE projects SET name = $1 WHERE project_id = $2 RETURNING *",
        )
        .bind(&params.name)
        .bind(project_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn delete_project(&self, project_id: &ProjectId) -> Result<bool, String> {
        let result = sqlx::query("DELETE FROM projects WHERE project_id = $1")
            .bind(project_id)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(result.rows_affected() > 0)
    }

    async fn list_projects_for_user(&self, user_id: &str) -> Result<Vec<DbProject>, String> {
        sqlx::query_as::<_, DbProject>(
            r#"
            SELECT p.* FROM projects p
            JOIN memberships m ON m.project_id = p.project_id
            WHERE m.user_id = $1
            ORDER BY p.created_at DESC
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }
}
