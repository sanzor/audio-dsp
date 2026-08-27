use domain::db::{DbTransformGrant, TransformId};
use sqlx::PgPool;

use crate::transform_grants::transform_grants_provider::CreateGrantParams;

use super::transform_grants_data_provider::TransformGrantsDataProvider;

pub struct PostgresTransformGrantsDataProvider {
    pool: PgPool,
}

impl PostgresTransformGrantsDataProvider {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl TransformGrantsDataProvider for PostgresTransformGrantsDataProvider {
    async fn create_grant(&self, params: CreateGrantParams) -> Result<DbTransformGrant, String> {
        sqlx::query_as::<_, DbTransformGrant>(
            r#"
            INSERT INTO transform_grants (transform_id, grantee_user_id, grantee_workspace_id, granted_by)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
        )
        .bind(params.transform_id)
        .bind(params.grantee_user_id)
        .bind(params.grantee_workspace_id)
        .bind(params.granted_by)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn delete_grant(&self, transform_id: TransformId, grant_id: i64) -> Result<bool, String> {
        let result = sqlx::query(
            "DELETE FROM transform_grants WHERE transform_id = $1 AND grant_id = $2",
        )
        .bind(transform_id)
        .bind(grant_id)
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(result.rows_affected() > 0)
    }

    async fn list_grants(&self, transform_id: TransformId) -> Result<Vec<DbTransformGrant>, String> {
        sqlx::query_as::<_, DbTransformGrant>(
            "SELECT * FROM transform_grants WHERE transform_id = $1 ORDER BY created_at DESC",
        )
        .bind(transform_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn has_access(&self, transform_id: TransformId, user_id: domain::domain_user::UserId) -> Result<bool, String> {
        sqlx::query_scalar::<_, bool>(
            r#"
            SELECT EXISTS (
                SELECT 1 FROM transform_grants g
                WHERE g.transform_id = $1
                  AND (
                    g.grantee_user_id = $2
                    OR g.grantee_workspace_id IN (
                        SELECT workspace_id FROM workspace_members WHERE user_id = $2
                    )
                  )
            )
            "#,
        )
        .bind(transform_id)
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }
}
