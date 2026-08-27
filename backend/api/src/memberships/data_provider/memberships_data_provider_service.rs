use domain::{db::DbMembership, domain_user::UserId, workspace_role::WorkspaceRole};
use sqlx::PgPool;

use crate::memberships::memberships_provider::CreateMembershipParams;

use super::memberships_data_provider::MembershipsDataProvider;

pub struct PostgresMembershipsDataProvider {
    pool: PgPool,
}

impl PostgresMembershipsDataProvider {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl MembershipsDataProvider for PostgresMembershipsDataProvider {
    async fn create_membership(
        &self,
        params: CreateMembershipParams,
    ) -> Result<DbMembership, String> {
        sqlx::query_as::<_, DbMembership>(
            r#"
            INSERT INTO workspace_members (workspace_id, user_id, role)
            VALUES ($1, $2, $3)
            ON CONFLICT (workspace_id, user_id) DO UPDATE SET role = EXCLUDED.role
            RETURNING *
            "#,
        )
        .bind(params.workspace_id)
        .bind(params.user_id)
        .bind(&params.role)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn delete_membership(&self, workspace_id: i32, user_id: UserId) -> Result<bool, String> {
        let result =
            sqlx::query("DELETE FROM workspace_members WHERE workspace_id = $1 AND user_id = $2")
                .bind(workspace_id)
                .bind(user_id)
                .execute(&self.pool)
                .await
                .map_err(|e| e.to_string())?;
        Ok(result.rows_affected() > 0)
    }

    async fn get_membership(
        &self,
        workspace_id: i32,
        user_id: UserId,
    ) -> Result<Option<DbMembership>, String> {
        sqlx::query_as::<_, DbMembership>(
            "SELECT * FROM workspace_members WHERE workspace_id = $1 AND user_id = $2",
        )
        .bind(workspace_id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn list_memberships(
        &self,
        workspace_id: Option<i32>,
        user_id: Option<UserId>,
    ) -> Result<Vec<DbMembership>, String> {
        match (workspace_id, user_id) {
            (Some(p), Some(u)) => sqlx::query_as::<_, DbMembership>(
                "SELECT * FROM workspace_members WHERE workspace_id = $1 AND user_id = $2",
            )
            .bind(p)
            .bind(u)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string()),
            (Some(p), None) => sqlx::query_as::<_, DbMembership>(
                "SELECT * FROM workspace_members WHERE workspace_id = $1",
            )
            .bind(p)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string()),
            (None, Some(u)) => sqlx::query_as::<_, DbMembership>(
                "SELECT * FROM workspace_members WHERE user_id = $1",
            )
            .bind(u)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string()),
            (None, None) => sqlx::query_as::<_, DbMembership>("SELECT * FROM workspace_members")
                .fetch_all(&self.pool)
                .await
                .map_err(|e| e.to_string()),
        }
    }

    async fn get_role(
        &self,
        workspace_id: i32,
        user_id: UserId,
    ) -> Result<Option<WorkspaceRole>, String> {
        let row: Option<(WorkspaceRole,)> = sqlx::query_as(
            "SELECT role FROM workspace_members WHERE workspace_id = $1 AND user_id = $2",
        )
        .bind(workspace_id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(row.map(|(r,)| r))
    }

    async fn update_role(
        &self,
        workspace_id: i32,
        user_id: UserId,
        role: WorkspaceRole,
    ) -> Result<Option<DbMembership>, String> {
        sqlx::query_as::<_, DbMembership>(
            r#"
            UPDATE workspace_members SET role = $3
            WHERE workspace_id = $1 AND user_id = $2
            RETURNING *
            "#,
        )
        .bind(workspace_id)
        .bind(user_id)
        .bind(&role)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }
}
