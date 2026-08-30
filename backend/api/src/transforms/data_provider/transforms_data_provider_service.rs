use domain::{
    db::{
        db_transform::{DbTransform, TransformId},
        ticket::{
            create_ticket_params::CreateTicketParams,
            db_resource::{DbResource, ResourceId},
            db_ticket::{DbTicket, TicketId},
            ticket_status::TicketStatus,
            update_ticket_params::UpdateTicketParams,
        },
    },
    domain_user::UserId,
};
use sqlx::PgPool;

use crate::domain::data_error::DataError;

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
    source_code: Option<String>,
    wasm_bytecode: Vec<u8>,
    name: String,
    description: Option<String>,
}

impl From<DbResourceRow> for DbResource {
    fn from(row: DbResourceRow) -> Self {
        Self {
            id: row.id,
            ticket_id: row.ticket_id,
            source_code: row.source_code,
            wasm_bytecode: row.wasm_bytecode,
            name: row.name,
            description: row.description,
        }
    }
}

#[derive(sqlx::FromRow)]
struct DbTransformRow {
    transform_id: TransformId,
    name: String,
    description: Option<String>,
    icon: Option<String>,
    kind: String,
    source_code: Option<String>,
    wasm_bytecode: Option<Vec<u8>>,
    metadata: Option<String>,
    owner_user_id: UserId,
    created_at: chrono::DateTime<chrono::Utc>,
}

impl From<DbTransformRow> for DbTransform {
    fn from(row: DbTransformRow) -> Self {
        Self {
            transform_id: row.transform_id,
            name: row.name,
            description: row.description,
            icon: row.icon,
            kind: row.kind,
            source_code: row.source_code,
            wasm_bytecode: row.wasm_bytecode,
            metadata: row.metadata,
            owner_user_id: row.owner_user_id,
            created_at: row.created_at,
        }
    }
}

const TRANSFORM_ROW_COLUMNS: &str = "t.transform_id, t.name, t.description, t.icon, t.kind, t.source_code, t.wasm_bytecode, t.metadata, t.owner_user_id, t.created_at";

#[async_trait::async_trait]
impl TransformsDataProvider for PostgresTransformsDataProvider {
    async fn create_transform_ticket(&self, ticket: CreateTicketParams) -> Result<DbTicket, DataError> {
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

    async fn get_ticket_transform_id(&self, ticket_id: TicketId) -> Result<TransformId, DataError> {
        sqlx::query_scalar::<_, TransformId>(
            "SELECT transform_id FROM transform_ticket WHERE ticket_id = $1",
        )
        .bind(ticket_id)
        .fetch_one(&self.pool)
        .await
        .map_err(DataError::from)
    }

    async fn get_resource_transform_id(&self, resource_id: ResourceId) -> Result<TransformId, DataError> {
        sqlx::query_scalar::<_, TransformId>(
            r#"
            SELECT tt.transform_id
            FROM transform_resource tr
            JOIN transform_ticket tt ON tt.ticket_id = tr.ticket_id
            WHERE tr.resource_id = $1
            "#,
        )
        .bind(resource_id)
        .fetch_one(&self.pool)
        .await
        .map_err(DataError::from)
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
        metadata: String,
    ) -> Result<DbResource, DataError> {
        let row = sqlx::query_as::<_, DbResourceRow>(
            r#"
            INSERT INTO transform_resource (ticket_id, wasm_bytecode, name, description, metadata)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING resource_id AS id, ticket_id, NULL::TEXT AS source_code, wasm_bytecode, name, description
            "#,
        )
        .bind(ticket_id)
        .bind(wasm_bytecode)
        .bind(name)
        .bind(description)
        .bind(metadata)
        .fetch_one(&self.pool)
        .await?;

        Ok(row.into())
    }

    async fn get_compiled_transform(&self, resource_id: ResourceId) -> Result<DbResource, DataError> {
        let row = sqlx::query_as::<_, DbResourceRow>(
            r#"
            SELECT tr.resource_id AS id, tr.ticket_id, tt.source_code, tr.wasm_bytecode, tr.name, tr.description
            FROM transform_resource tr
            JOIN transform_ticket tt ON tt.ticket_id = tr.ticket_id
            WHERE tr.resource_id = $1
            "#,
        )
        .bind(resource_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(row.into())
    }

    async fn list_transform_summaries(&self, offset: i64, limit: i64) -> Result<(Vec<DbTransform>, i64), DataError> {
        #[derive(sqlx::FromRow)]
        struct Row {
            transform_id: TransformId,
            name: String,
            description: Option<String>,
            icon: Option<String>,
            kind: String,
            source_code: Option<String>,
            wasm_bytecode: Option<Vec<u8>>,
            metadata: Option<String>,
            owner_user_id: UserId,
            created_at: chrono::DateTime<chrono::Utc>,
            total: i64,
        }

        let rows = sqlx::query_as::<_, Row>(
            r#"
            SELECT
                t.transform_id, t.name, t.description, t.icon, t.kind, t.source_code, t.wasm_bytecode, t.metadata, t.owner_user_id,
                t.created_at,
                COUNT(*) OVER () AS total
            FROM transform t ORDER BY t.created_at DESC LIMIT $1 OFFSET $2
            "#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        let total = rows.first().map(|r| r.total).unwrap_or(0);
        let transforms = rows
            .into_iter()
            .map(|r| DbTransform {
                transform_id: r.transform_id,
                name: r.name,
                description: r.description,
                icon: r.icon,
                kind: r.kind,
                source_code: r.source_code,
                wasm_bytecode: r.wasm_bytecode,
                metadata: r.metadata,
                owner_user_id: r.owner_user_id,
                created_at: r.created_at,
            })
            .collect();

        Ok((transforms, total))
    }

    async fn get_transforms_for_workspace_and_user(&self, user_id: UserId, workspace_id: domain::db::WorkspaceId) -> Result<Vec<DbTransform>, DataError> {
        let rows = sqlx::query_as::<_, DbTransformRow>(&format!(
            r#"
            SELECT DISTINCT {TRANSFORM_ROW_COLUMNS}
            FROM transform t
            WHERE t.owner_user_id = $1
               OR EXISTS (SELECT 1 FROM transform_grants g WHERE g.transform_id = t.transform_id AND g.grantee_user_id = $1)
               OR EXISTS (SELECT 1 FROM transform_grants g WHERE g.transform_id = t.transform_id AND g.grantee_workspace_id = $2)
            ORDER BY t.created_at DESC
            "#,
        ))
        .bind(user_id)
        .bind(workspace_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(DbTransform::from).collect())
    }

    async fn get_transform(&self, id: TransformId) -> Result<DbTransform, DataError> {
        let row = sqlx::query_as::<_, DbTransformRow>(&format!(
            r#"SELECT {TRANSFORM_ROW_COLUMNS} FROM transform t WHERE t.transform_id = $1"#,
        ))
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(row.into())
    }

    async fn get_transforms(&self, ids: &[TransformId]) -> Result<Vec<DbTransform>, DataError> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }

        let rows = sqlx::query_as::<_, DbTransformRow>(&format!(
            r#"
            SELECT {TRANSFORM_ROW_COLUMNS}
            FROM transform t
            WHERE t.transform_id = ANY($1)
            "#,
        ))
        .bind(ids)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(DbTransform::from).collect())
    }

    async fn get_transform_owner(&self, id: TransformId) -> Result<UserId, DataError> {
        sqlx::query_scalar::<_, UserId>("SELECT owner_user_id FROM transform WHERE transform_id = $1")
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .map_err(DataError::from)
    }

    async fn list_accessible_transform_ids(&self, user_id: UserId) -> Result<Vec<TransformId>, DataError> {
        sqlx::query_scalar::<_, TransformId>(
            r#"
            SELECT t.transform_id
            FROM transform t
            WHERE t.owner_user_id = $1
               OR EXISTS (SELECT 1 FROM transform_grants g WHERE g.transform_id = t.transform_id AND g.grantee_user_id = $1)
               OR EXISTS (
                    SELECT 1 FROM transform_grants g
                    WHERE g.transform_id = t.transform_id
                      AND g.grantee_workspace_id IN (SELECT workspace_id FROM workspace_members WHERE user_id = $1)
               )
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .map_err(DataError::from)
    }

    async fn delete_transform(&self, id: TransformId) -> Result<(), DataError> {
        // Draft deletion is only allowed for transforms that have never been
        // published — a published transform may already be referenced by
        // editor graphs (graph_state stores a bare transform_id with no
        // version pin), so deleting it out from under them is not safe.
        // "Published" is now just "the live row has a non-empty
        // wasm_bytecode" — there's no separate transform_binary table to
        // check against any more.
        // See agents/decisions/0002-transform-draft-lifecycle-decisions.md.
        let is_published: bool = sqlx::query_scalar(
            "SELECT octet_length(wasm_bytecode) > 0 FROM transform WHERE transform_id = $1",
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        if is_published {
            return Err(DataError::Conflict(
                "transform has been published at least once and cannot be deleted".to_string(),
            ));
        }

        // transform_draft, transform_ticket (-> transform_resource) all have
        // ON DELETE CASCADE from transform already.
        let result = sqlx::query(r#"DELETE FROM transform WHERE transform_id = $1"#)
            .bind(id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(DataError::NotFound);
        }

        Ok(())
    }
}
