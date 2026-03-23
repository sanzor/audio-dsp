use domain::db::db_transform::{DbTransform, DbTransformPort, TransformId};
use sqlx::PgPool;

use super::transforms_data_provider::TransformsDataProvider;

pub struct PostgresTransformsDataProvider {
    pool: PgPool,
}

impl PostgresTransformsDataProvider {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl TransformsDataProvider for PostgresTransformsDataProvider {
    async fn get_transform(&self, id: TransformId) -> Result<DbTransform, String> {
        sqlx::query_as::<_, DbTransform>(
            r#"SELECT transform_id, name, description, icon, created_at FROM transforms WHERE transform_id = $1"#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn get_transforms_paginated(&self, offset: i64, limit: i64) -> Result<(Vec<DbTransform>, i64), String> {
        let rows = sqlx::query_as::<_, DbTransform>(
            r#"SELECT transform_id, name, description, icon, created_at FROM transforms ORDER BY created_at DESC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        let total: i64 = sqlx::query_scalar(r#"SELECT COUNT(*) FROM transforms"#)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok((rows, total))
    }

    async fn get_ports_for_transform(&self, id: TransformId) -> Result<Vec<DbTransformPort>, String> {
        sqlx::query_as::<_, DbTransformPort>(
            r#"SELECT port_id, transform_id, name, direction, port_order, description FROM transform_ports WHERE transform_id = $1 ORDER BY direction, port_order"#,
        )
        .bind(id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn insert_transform(
        &self,
        name: String,
        description: Option<String>,
        icon: Option<String>,
    ) -> Result<DbTransform, String> {
        sqlx::query_as::<_, DbTransform>(
            r#"INSERT INTO transforms (name, description, icon) VALUES ($1, $2, $3)
               RETURNING transform_id, name, description, icon, created_at"#,
        )
        .bind(name)
        .bind(description)
        .bind(icon)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn update_transform(
        &self,
        id: TransformId,
        name: String,
        description: Option<String>,
        icon: Option<String>,
    ) -> Result<DbTransform, String> {
        sqlx::query_as::<_, DbTransform>(
            r#"UPDATE transforms SET name = $2, description = $3, icon = $4 WHERE transform_id = $1
               RETURNING transform_id, name, description, icon, created_at"#,
        )
        .bind(id)
        .bind(name)
        .bind(description)
        .bind(icon)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn delete_transform(&self, id: TransformId) -> Result<(), String> {
        sqlx::query(r#"DELETE FROM transforms WHERE transform_id = $1"#)
            .bind(id)
            .execute(&self.pool)
            .await
            .map(|_| ())
            .map_err(|e| e.to_string())
    }

    async fn insert_port(
        &self,
        transform_id: TransformId,
        name: String,
        direction: String,
        port_order: i32,
        description: Option<String>,
    ) -> Result<DbTransformPort, String> {
        sqlx::query_as::<_, DbTransformPort>(
            r#"INSERT INTO transform_ports (transform_id, name, direction, port_order, description)
               VALUES ($1, $2, $3, $4, $5)
               RETURNING port_id, transform_id, name, direction, port_order, description"#,
        )
        .bind(transform_id)
        .bind(name)
        .bind(direction)
        .bind(port_order)
        .bind(description)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn delete_port(&self, port_id: i64) -> Result<(), String> {
        sqlx::query(r#"DELETE FROM transform_ports WHERE port_id = $1"#)
            .bind(port_id)
            .execute(&self.pool)
            .await
            .map(|_| ())
            .map_err(|e| e.to_string())
    }
}
