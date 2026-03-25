use domain::{db::DbMembership, project_role::ProjectRole};
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
            INSERT INTO project_members (project_id, user_id, role)
            VALUES ($1, $2, $3)
            ON CONFLICT (project_id, user_id) DO UPDATE SET role = EXCLUDED.role
            RETURNING *
            "#,
        )
        .bind(params.project_id)
        .bind(params.user_id)
        .bind(&params.role)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn delete_membership(&self, project_id: i32, user_id: i32) -> Result<bool, String> {
        let result =
            sqlx::query("DELETE FROM project_members WHERE project_id = $1 AND user_id = $2")
                .bind(project_id)
                .bind(user_id)
                .execute(&self.pool)
                .await
                .map_err(|e| e.to_string())?;
        Ok(result.rows_affected() > 0)
    }

    async fn get_membership(
        &self,
        project_id: i32,
        user_id: i32,
    ) -> Result<Option<DbMembership>, String> {
        sqlx::query_as::<_, DbMembership>(
            "SELECT * FROM project_members WHERE project_id = $1 AND user_id = $2",
        )
        .bind(project_id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn list_memberships(
        &self,
        project_id: Option<i32>,
        user_id: Option<i32>,
    ) -> Result<Vec<DbMembership>, String> {
        match (project_id, user_id) {
            (Some(p), Some(u)) => sqlx::query_as::<_, DbMembership>(
                "SELECT * FROM project_members WHERE project_id = $1 AND user_id = $2",
            )
            .bind(p)
            .bind(u)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string()),
            (Some(p), None) => sqlx::query_as::<_, DbMembership>(
                "SELECT * FROM project_members WHERE project_id = $1",
            )
            .bind(p)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string()),
            (None, Some(u)) => sqlx::query_as::<_, DbMembership>(
                "SELECT * FROM project_members WHERE user_id = $1",
            )
            .bind(u)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string()),
            (None, None) => sqlx::query_as::<_, DbMembership>("SELECT * FROM project_members")
                .fetch_all(&self.pool)
                .await
                .map_err(|e| e.to_string()),
        }
    }

    async fn get_role(
        &self,
        project_id: i32,
        user_id: i32,
    ) -> Result<Option<ProjectRole>, String> {
        let row: Option<(ProjectRole,)> = sqlx::query_as(
            "SELECT role FROM project_members WHERE project_id = $1 AND user_id = $2",
        )
        .bind(project_id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(row.map(|(r,)| r))
    }

    async fn update_role(
        &self,
        project_id: i32,
        user_id: i32,
        role: ProjectRole,
    ) -> Result<Option<DbMembership>, String> {
        sqlx::query_as::<_, DbMembership>(
            r#"
            UPDATE project_members SET role = $3
            WHERE project_id = $1 AND user_id = $2
            RETURNING *
            "#,
        )
        .bind(project_id)
        .bind(user_id)
        .bind(&role)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }
}
