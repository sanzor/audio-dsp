use domain::db::{
    db_transform::{DbTransform, DbTransformDefinition, DbTransformParam, DbTransformPort, TransformId},
    ticket::{create_ticket_params::CreateTicketParams, db_resource::{DbResource, ResourceId}, db_ticket::{DbTicket, TicketId}, ticket_status::TicketStatus, update_ticket_params::UpdateTicketParams},
    transform_draft::DbTransformDraft,
    transform_snapshot::{ParamSnapshot, PortSnapshot},
};
use sqlx::PgPool;

use crate::{domain::data_error::DataError};

use super::transforms_data_provider::{NewTransformParam, NewTransformPort, TransformsDataProvider};

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
    kind: String,
    source_code: Option<String>,
    ports: sqlx::types::Json<Vec<DbTransformPort>>,
    params: sqlx::types::Json<Vec<DbTransformParam>>,
}

#[derive(sqlx::FromRow)]
struct DbTicketRow {
    id: TicketId,
    issued_by: i64,
    status: String,
    resource_id: Option<i64>,
    error_message: Option<String>,
    timestamp: i64,
}

impl TryFrom<DbTicketRow> for DbTicket {
    type Error = DataError;

    fn try_from(value: DbTicketRow) -> Result<Self, Self::Error> {
        let status = match value.status.as_str() {
            "processing" => TicketStatus::Processing,
            "failed" => TicketStatus::Failed {
                message: value.error_message.unwrap_or_default(),
            },
            "successful" => {
                let resource_id = value.resource_id.ok_or_else(|| {
                    DataError::Internal("successful ticket is missing its resource".to_string())
                })?;
                TicketStatus::Successful { resource_id }
            }
            other => {
                return Err(DataError::Internal(format!(
                    "unsupported transform ticket status: {other}"
                )))
            }
        };

        Ok(DbTicket {
            id: value.id,
            issued_by: value.issued_by,
            status,
            timestamp: value.timestamp,
        })
    }
}

#[derive(sqlx::FromRow)]
struct DbResourceRow {
    id: ResourceId,
    ticket_id: TicketId,
    wasm_bytecode: Vec<u8>,
    name: String,
    description: Option<String>,
    ports: sqlx::types::Json<Vec<PortSnapshot>>,
    params: sqlx::types::Json<Vec<ParamSnapshot>>,
}

impl From<DbResourceRow> for DbResource {
    fn from(row: DbResourceRow) -> Self {
        Self {
            id: row.id,
            ticket_id: row.ticket_id,
            wasm_bytecode: row.wasm_bytecode,
            name: row.name,
            description: row.description,
            ports: row.ports.0,
            params: row.params.0,
        }
    }
}

#[derive(sqlx::FromRow)]
struct DbTransformDraftRow {
    transform_id: TransformId,
    source_code: String,
    wasm_bytecode: Option<Vec<u8>>,
    wasm_source_code: Option<String>,
    name: Option<String>,
    description: Option<String>,
    ports: Option<sqlx::types::Json<Vec<PortSnapshot>>>,
    params: Option<sqlx::types::Json<Vec<ParamSnapshot>>>,
}

impl From<DbTransformDraftRow> for DbTransformDraft {
    fn from(row: DbTransformDraftRow) -> Self {
        Self {
            transform_id: row.transform_id,
            source_code: row.source_code,
            wasm_bytecode: row.wasm_bytecode,
            wasm_source_code: row.wasm_source_code,
            name: row.name,
            description: row.description,
            ports: row.ports.map(|j| j.0).unwrap_or_default(),
            params: row.params.map(|j| j.0).unwrap_or_default(),
        }
    }
}

#[async_trait::async_trait]
impl TransformsDataProvider for PostgresTransformsDataProvider {
    async fn create_ticket(&self, ticket: CreateTicketParams) -> Result<DbTicket, DataError> {
        let row = sqlx::query_as::<_, DbTicketRow>(
            r#"
            INSERT INTO transform_ticket (transform_id, issued_by, source_code, status)
            VALUES ($1, $2, $3, 'processing')
            RETURNING
                ticket_id AS id,
                issued_by,
                status,
                NULL::BIGINT AS resource_id,
                error_message,
                EXTRACT(EPOCH FROM created_at)::BIGINT AS timestamp
            "#,
        )
        .bind(ticket.transform_id)
        .bind(ticket.user_id)
        .bind(ticket.source_code)
        .fetch_one(&self.pool)
        .await?;

        DbTicket::try_from(row)
    }

    async fn get_ticket(&self, ticket_id: TicketId) -> Result<DbTicket, DataError> {
        let row = sqlx::query_as::<_, DbTicketRow>(
            r#"
            SELECT
                tt.ticket_id AS id,
                tt.issued_by,
                tt.status,
                tr.resource_id,
                tt.error_message,
                EXTRACT(EPOCH FROM tt.created_at)::BIGINT AS timestamp
            FROM transform_ticket tt
            LEFT JOIN transform_resource tr ON tr.ticket_id = tt.ticket_id
            WHERE tt.ticket_id = $1
            "#,
        )
        .bind(ticket_id)
        .fetch_one(&self.pool)
        .await?;

        DbTicket::try_from(row)
    }

    async fn remove_ticket(&self, ticket_id: TicketId) -> Result<(), DataError> {
        let result = sqlx::query(r#"DELETE FROM transform_ticket WHERE ticket_id = $1"#)
            .bind(ticket_id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            Err(DataError::NotFound)
        } else {
            Ok(())
        }
    }

    async fn update_ticket(&self, params: UpdateTicketParams) -> Result<DbTicket, DataError> {
        let status_str = match &params.status {
            TicketStatus::Processing => "processing",
            TicketStatus::Failed { .. } => "failed",
            TicketStatus::Successful { .. } => "successful",
        };
        let error_message = match &params.status {
            TicketStatus::Failed { message } => Some(message.clone()),
            _ => None,
        };

        let row = sqlx::query_as::<_, DbTicketRow>(
            r#"
            WITH updated AS (
                UPDATE transform_ticket
                SET status = $2, error_message = $3
                WHERE ticket_id = $1
                RETURNING ticket_id, issued_by, status, error_message, created_at
            )
            SELECT
                u.ticket_id AS id,
                u.issued_by,
                u.status,
                tr.resource_id,
                u.error_message,
                EXTRACT(EPOCH FROM u.created_at)::BIGINT AS timestamp
            FROM updated u
            LEFT JOIN transform_resource tr ON tr.ticket_id = u.ticket_id
            "#,
        )
        .bind(params.ticket_id)
        .bind(status_str)
        .bind(error_message)
        .fetch_one(&self.pool)
        .await?;

        DbTicket::try_from(row)
    }

    async fn create_resource(
        &self,
        ticket_id: TicketId,
        wasm_bytecode: Vec<u8>,
        name: String,
        description: Option<String>,
        ports: Vec<NewTransformPort>,
        params: Vec<NewTransformParam>,
    ) -> Result<DbResource, DataError> {
        let ports: Vec<PortSnapshot> = ports.into_iter().map(PortSnapshot::from).collect();
        let params: Vec<ParamSnapshot> = params.into_iter().map(ParamSnapshot::from).collect();

        let row = sqlx::query_as::<_, DbResourceRow>(
            r#"
            INSERT INTO transform_resource (ticket_id, wasm_bytecode, name, description, ports, params)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING resource_id AS id, ticket_id, wasm_bytecode, name, description, ports, params
            "#,
        )
        .bind(ticket_id)
        .bind(wasm_bytecode)
        .bind(name)
        .bind(description)
        .bind(sqlx::types::Json(ports))
        .bind(sqlx::types::Json(params))
        .fetch_one(&self.pool)
        .await?;

        Ok(row.into())
    }

    async fn get_resource(&self, resource_id: ResourceId) -> Result<DbResource, DataError> {
        let row = sqlx::query_as::<_, DbResourceRow>(
            r#"
            SELECT resource_id AS id, ticket_id, wasm_bytecode, name, description, ports, params
            FROM transform_resource
            WHERE resource_id = $1
            "#,
        )
        .bind(resource_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(row.into())
    }

    async fn update_resource(&self, resource_id: ResourceId, ticket_id: TicketId) -> Result<DbResource, DataError> {
        let row = sqlx::query_as::<_, DbResourceRow>(
            r#"
            UPDATE transform_resource
            SET ticket_id = $2
            WHERE resource_id = $1
            RETURNING resource_id AS id, ticket_id, wasm_bytecode, name, description, ports, params
            "#,
        )
        .bind(resource_id)
        .bind(ticket_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(row.into())
    }

    async fn remove_resource(&self, resource_id: ResourceId) -> Result<(), DataError> {
        let result = sqlx::query(r#"DELETE FROM transform_resource WHERE resource_id = $1"#)
            .bind(resource_id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            Err(DataError::NotFound)
        } else {
            Ok(())
        }
    }

    async fn get_transform(&self, id: TransformId) -> Result<DbTransform, String> {
        sqlx::query_as::<_, DbTransform>(
            r#"SELECT transform_id, name, description, icon, kind, created_at FROM transform WHERE transform_id = $1"#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn get_current_ports(&self, transform_id: TransformId) -> Result<Vec<DbTransformPort>, DataError> {
        let rows = sqlx::query_as::<_, DbTransformPort>(
            r#"
            SELECT port_id, transform_id, name, direction, port_order, description, kind, cardinality
            FROM transform_port
            WHERE transform_id = $1
            ORDER BY CASE WHEN direction = 'input' THEN 0 ELSE 1 END, port_order, port_id
            "#,
        )
        .bind(transform_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
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
            kind: row.kind,
            source_code: row.source_code,
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
                kind: row.kind,
                source_code: row.source_code,
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
            kind: String,
            created_at: chrono::DateTime<chrono::Utc>,
            total: i64,
        }

        let rows = sqlx::query_as::<_, Row>(
            r#"SELECT transform_id, name, description, icon, kind, created_at, COUNT(*) OVER () AS total
               FROM transform ORDER BY created_at DESC LIMIT $1 OFFSET $2"#,
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
            kind: r.kind,
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
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        let transform = sqlx::query_as::<_, DbTransform>(
            r#"INSERT INTO transform (name, description, icon, kind) VALUES ($1, $2, $3, 'primitive')
               RETURNING transform_id, name, description, icon, kind, created_at"#,
        )
        .bind(name)
        .bind(description)
        .bind(icon)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        sqlx::query(r#"INSERT INTO transform_draft (transform_id) VALUES ($1)"#)
            .bind(transform.transform_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;

        Ok(transform)
    }

    async fn get_draft(&self, transform_id: TransformId) -> Result<DbTransformDraft, DataError> {
        let row = sqlx::query_as::<_, DbTransformDraftRow>(
            r#"
            SELECT transform_id, source_code, wasm_bytecode, wasm_source_code, name, description, ports, params
            FROM transform_draft
            WHERE transform_id = $1
            "#,
        )
        .bind(transform_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(row.into())
    }

    async fn save_transform_draft(
        &self,
        transform_id: TransformId,
        source_code: String,
        resource_id: Option<ResourceId>,
    ) -> Result<DbTransformDraft, DataError> {
        let mut tx = self.pool.begin().await?;

        sqlx::query(r#"UPDATE transform_draft SET source_code = $2, updated_at = now() WHERE transform_id = $1"#)
            .bind(transform_id)
            .bind(&source_code)
            .execute(&mut *tx)
            .await?;

        if let Some(resource_id) = resource_id {
            // Provenance check, in the same query as the "belongs to this
            // transform" check: the resource's own compile ticket's
            // source_code must equal the source_code being saved right now
            // (t.source_code = $3). This is what actually guarantees
            // transform_draft never ends up with a binary that doesn't
            // correspond to its own saved source — the frontend's
            // `attachableResourceId` guard (code-editor.tsx) already only
            // ever sends a resource_id while the editor buffer still matches
            // what was compiled, so a mismatch reaching here at all means a
            // stale or buggy client. We treat that as a hard validation
            // error (rolling back the whole save, including the source-only
            // part) rather than silently downgrading to a source-only save —
            // consistent with the pre-existing "resource does not belong to
            // this transform" case below, which already fails the same way.
            // wasm_source_code is set to the same value we just validated
            // t.source_code equals, so it never has to be read back out.
            let result = sqlx::query(
                r#"
                UPDATE transform_draft ss
                SET wasm_bytecode = r.wasm_bytecode,
                    wasm_source_code = $3,
                    name = r.name,
                    description = r.description,
                    ports = r.ports,
                    params = r.params,
                    updated_at = now()
                FROM transform_resource r
                JOIN transform_ticket t ON t.ticket_id = r.ticket_id
                WHERE ss.transform_id = $1
                  AND r.resource_id = $2
                  AND t.transform_id = $1
                  AND t.source_code = $3
                "#,
            )
            .bind(transform_id)
            .bind(resource_id)
            .bind(&source_code)
            .execute(&mut *tx)
            .await?;

            if result.rows_affected() == 0 {
                return Err(DataError::Validation(
                    "resource_id does not correspond to a compile of the exact source being saved (foreign, stale, or mismatched resource)".to_string(),
                ));
            }
        }

        let row = sqlx::query_as::<_, DbTransformDraftRow>(
            r#"
            SELECT transform_id, source_code, wasm_bytecode, wasm_source_code, name, description, ports, params
            FROM transform_draft
            WHERE transform_id = $1
            "#,
        )
        .bind(transform_id)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(row.into())
    }

    async fn delete_transform(&self, id: TransformId) -> Result<(), DataError> {
        // Draft deletion is only allowed for transforms that have never been
        // published — a published transform may already be referenced by
        // editor graphs (graph_state stores a bare transform_id with no
        // version pin), so deleting it out from under them is not safe.
        // See agents/decisions/0002-transform-draft-lifecycle-decisions.md.
        let published_count: i64 = sqlx::query_scalar(
            r#"SELECT COUNT(*) FROM transform_binary WHERE transform_id = $1"#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        if published_count > 0 {
            return Err(DataError::Conflict(
                "transform has been published at least once and cannot be deleted".to_string(),
            ));
        }

        // transform_draft, transform_ticket (-> transform_resource),
        // transform_port, and transform_param all have ON DELETE CASCADE
        // from transforms already (migrations 0004/0009/0011/0014) — no
        // schema change needed for the cascade itself.
        let result = sqlx::query(r#"DELETE FROM transform WHERE transform_id = $1"#)
            .bind(id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(DataError::NotFound);
        }

        Ok(())
    }

    async fn publish_compiled_transform(
        &self,
        transform_id: TransformId,
        wasm_bytecode: Vec<u8>,
        source_code: String,
        name: String,
        description: Option<String>,
        ports: Vec<NewTransformPort>,
        params: Vec<NewTransformParam>,
    ) -> Result<(), String> {
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        sqlx::query(r#"UPDATE transform SET name = $2, description = $3 WHERE transform_id = $1"#)
            .bind(transform_id)
            .bind(&name)
            .bind(&description)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

        sqlx::query(r#"DELETE FROM transform_port WHERE transform_id = $1"#)
            .bind(transform_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

        sqlx::query(r#"DELETE FROM transform_param WHERE transform_id = $1"#)
            .bind(transform_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

        for port in &ports {
            sqlx::query(
                r#"INSERT INTO transform_port (transform_id, name, direction, port_order, description, kind, cardinality)
                   VALUES ($1, $2, $3, $4, $5, $6, $7)"#,
            )
            .bind(transform_id)
            .bind(&port.name)
            .bind(&port.direction)
            .bind(port.order)
            .bind(&port.description)
            .bind(&port.kind)
            .bind(&port.cardinality)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;
        }

        for param in &params {
            sqlx::query(
                r#"INSERT INTO transform_param (transform_id, name, param_order, default_value, min_value, max_value, description)
                   VALUES ($1, $2, $3, $4, $5, $6, $7)"#,
            )
            .bind(transform_id)
            .bind(&param.name)
            .bind(param.order)
            .bind(param.default_value)
            .bind(param.min_value)
            .bind(param.max_value)
            .bind(&param.description)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;
        }

        sqlx::query(
            r#"INSERT INTO transform_binary (transform_id, wasm_bytecode, source)
               VALUES ($1, $2, $3)
               ON CONFLICT (transform_id)
               DO UPDATE SET wasm_bytecode = EXCLUDED.wasm_bytecode, source = EXCLUDED.source, updated_at = now()"#,
        )
        .bind(transform_id)
        .bind(&wasm_bytecode)
        .bind(&source_code)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;

        Ok(())
    }
}
