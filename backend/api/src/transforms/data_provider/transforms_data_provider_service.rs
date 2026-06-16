use domain::db::{db_transform::{DbTransform, DbTransformDefinition, DbTransformParam, DbTransformPort, TransformId}, ticket::db_ticket::DbTicket};
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

#[derive(sqlx::FromRow)]
struct DbTransformDefinitionRow {
    transform_id: TransformId,
    name: String,
    description: Option<String>,
    icon: Option<String>,
    ports: sqlx::types::Json<Vec<DbTransformPort>>,
    params: sqlx::types::Json<Vec<DbTransformParam>>,
}

#[async_trait::async_trait]
impl TransformsDataProvider for PostgresTransformsDataProvider {

    async fn create_ticket(&self,ticket:crate::transforms::ticket::create_ticket_params::CreateTicketParams)->Result<DbTicket,DataError>{
    {
        sqlx::query_as::<_,DbTicket>()
    }
    async fn get_transform(&self, id: TransformId) -> Result<DbTransform, String> {
        sqlx::query_as::<_, DbTransform>(
            r#"SELECT transform_id, name, description, icon, created_at FROM transforms WHERE transform_id = $1"#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn get_transform_definition(&self, id: TransformId) -> Result<DbTransformDefinition, String> {
        let row = sqlx::query_as::<_, DbTransformDefinitionRow>(
            r#"SELECT * FROM get_transform_definition($1)"#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(DbTransformDefinition {
            transform_id: row.transform_id,
            name: row.name,
            description: row.description,
            icon: row.icon,
            ports: row.ports.0,
            params: row.params.0,
        })
    }

    async fn get_transform_definitions(&self, ids: &[TransformId]) -> Result<Vec<DbTransformDefinition>, String> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }

        let rows = sqlx::query_as::<_, DbTransformDefinitionRow>(
            r#"
            SELECT def.*
            FROM unnest($1::bigint[]) WITH ORDINALITY AS requested(transform_id, ord)
            JOIN LATERAL get_transform_definition(requested.transform_id) AS def ON true
            ORDER BY requested.ord
            "#,
        )
        .bind(ids)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(rows
            .into_iter()
            .map(|row| DbTransformDefinition {
                transform_id: row.transform_id,
                name: row.name,
                description: row.description,
                icon: row.icon,
                ports: row.ports.0,
                params: row.params.0,
            })
            .collect())
    }

    async fn list_transform_summaries(&self, offset: i64, limit: i64) -> Result<(Vec<DbTransform>, i64), String> {
        #[derive(sqlx::FromRow)]
        struct Row {
            transform_id: TransformId,
            name: String,
            description: Option<String>,
            icon: Option<String>,
            created_at: chrono::DateTime<chrono::Utc>,
            total: i64,
        }

        let rows = sqlx::query_as::<_, Row>(
            r#"SELECT transform_id, name, description, icon, created_at, COUNT(*) OVER () AS total
               FROM transforms ORDER BY created_at DESC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        let total = rows.first().map(|r| r.total).unwrap_or(0);
        let transforms = rows.into_iter().map(|r| DbTransform {
            transform_id: r.transform_id,
            name: r.name,
            description: r.description,
            icon: r.icon,
            created_at: r.created_at,
        }).collect();

        Ok((transforms, total))
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
