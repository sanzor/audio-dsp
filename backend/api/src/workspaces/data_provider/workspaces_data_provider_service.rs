use domain::db::{DbWorkspace, WorkspaceId};
use sqlx::PgPool;

use crate::workspaces::workspaces_provider::{CreateWorkspaceParams, UpdateWorkspaceParams};

use super::workspaces_data_provider::WorkspacesDataProvider;

pub struct PostgresWorkspacesDataProvider {
    pool: PgPool,
}

impl PostgresWorkspacesDataProvider {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl WorkspacesDataProvider for PostgresWorkspacesDataProvider {
    async fn create_workspace(&self, params: CreateWorkspaceParams) -> Result<DbWorkspace, String> {
        sqlx::query_as::<_, DbWorkspace>(
            "INSERT INTO workspaces (name, created_by) VALUES ($1, $2) RETURNING *",
        )
        .bind(&params.name)
        .bind(params.created_by)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn get_workspace(&self, workspace_id: &WorkspaceId) -> Result<Option<DbWorkspace>, String> {
        sqlx::query_as::<_, DbWorkspace>("SELECT * FROM workspaces WHERE workspace_id = $1")
            .bind(workspace_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| e.to_string())
    }

    async fn update_workspace(
        &self,
        workspace_id: &WorkspaceId,
        params: UpdateWorkspaceParams,
    ) -> Result<Option<DbWorkspace>, String> {
        sqlx::query_as::<_, DbWorkspace>(
            "UPDATE workspaces SET name = $1 WHERE workspace_id = $2 RETURNING *",
        )
        .bind(&params.name)
        .bind(workspace_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn delete_workspace(&self, workspace_id: &WorkspaceId) -> Result<bool, String> {
        let result = sqlx::query("DELETE FROM workspaces WHERE workspace_id = $1")
            .bind(workspace_id)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(result.rows_affected() > 0)
    }

    async fn list_workspaces_for_user(&self, user_id: i32) -> Result<Vec<DbWorkspace>, String> {
        sqlx::query_as::<_, DbWorkspace>(
            r#"
            SELECT w.* FROM workspaces w
            JOIN workspace_members m ON m.workspace_id = w.workspace_id
            WHERE m.user_id = $1
            ORDER BY w.created_at DESC
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }
}
