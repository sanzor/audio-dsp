use domain::{
    db::{
        db_transform::{DbTransform, TransformId},
        db_transform_draft::DbTransformDraft,
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

/// `metadata` is persisted as a JSON string (`transform.metadata` /
/// `transform_draft.metadata` / `transform_resource.metadata` are all TEXT
/// columns), but `DbTransform`/`DbTransformDraft` declare the field as
/// `Vec<u32>`, not `String` — that type was already committed to elsewhere
/// and this file isn't the place to change it. `Vec<u32>` has no natural
/// encoding for an arbitrary JSON blob, so this is a deliberately simple,
/// total, lossless stopgap: one `u32` per UTF-8 byte. See the conversation
/// report for why this almost certainly wants to just be `String` instead.
fn metadata_to_words(json: &str) -> Vec<u32> {
    json.bytes().map(u32::from).collect()
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
}

impl From<DbResourceRow> for DbResource {
    fn from(row: DbResourceRow) -> Self {
        Self {
            id: row.id,
            ticket_id: row.ticket_id,
            wasm_bytecode: row.wasm_bytecode,
            name: row.name,
            description: row.description,
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
    kind: String,
    metadata: String,
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
            kind: row.kind,
            metadata: metadata_to_words(&row.metadata),
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
    source_code: String,
    wasm_bytecode: Vec<u8>,
    metadata: String,
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
            metadata: metadata_to_words(&row.metadata),
            owner_user_id: row.owner_user_id,
            created_at: row.created_at,
        }
    }
}

const TRANSFORM_ROW_COLUMNS: &str = "t.transform_id, t.name, t.description, t.icon, t.kind, t.source_code, t.wasm_bytecode, t.metadata, t.owner_user_id, t.created_at";
const TRANSFORM_ROW_COLUMNS_BARE: &str = "transform_id, name, description, icon, kind, source_code, wasm_bytecode, metadata, owner_user_id, created_at";
const DRAFT_ROW_COLUMNS: &str = "transform_id, source_code, wasm_bytecode, wasm_source_code, name, description, kind, metadata";

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
            RETURNING resource_id AS id, ticket_id, wasm_bytecode, name, description
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
            SELECT resource_id AS id, ticket_id, wasm_bytecode, name, description
            FROM transform_resource
            WHERE resource_id = $1
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
            source_code: String,
            wasm_bytecode: Vec<u8>,
            metadata: String,
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
                metadata: metadata_to_words(&r.metadata),
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

    async fn insert_transform_draft(
        &self,
        name: String,
        description: Option<String>,
        icon: Option<String>,
        kind: String,
        owner_user_id: UserId,
    ) -> Result<DbTransformDraft, DataError> {
        let mut tx = self.pool.begin().await?;

        let transform_id: TransformId = sqlx::query_scalar(
            r#"INSERT INTO transform (name, description, icon, kind, owner_user_id, source_code, wasm_bytecode, metadata)
               VALUES ($1, $2, $3, $4, $5, '', ''::bytea, '')
               RETURNING transform_id"#,
        )
        .bind(&name)
        .bind(&description)
        .bind(&icon)
        .bind(&kind)
        .bind(owner_user_id)
        .fetch_one(&mut *tx)
        .await?;

        let row = sqlx::query_as::<_, DbTransformDraftRow>(&format!(
            r#"
            INSERT INTO transform_draft (transform_id, source_code, name, description, kind, metadata)
            VALUES ($1, '', $2, $3, $4, '')
            RETURNING {DRAFT_ROW_COLUMNS}
            "#,
        ))
        .bind(transform_id)
        .bind(&name)
        .bind(&description)
        .bind(&kind)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(row.into())
    }

    async fn get_transform_draft(&self, id: domain::db::db_transform_draft::TransformDraftId) -> Result<DbTransformDraft, DataError> {
        let row = sqlx::query_as::<_, DbTransformDraftRow>(&format!(
            r#"SELECT {DRAFT_ROW_COLUMNS} FROM transform_draft WHERE transform_id = $1"#,
        ))
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(row.into())
    }

    async fn get_transform_drafts(&self, ids: &[domain::db::db_transform_draft::TransformDraftId]) -> Result<Vec<DbTransformDraft>, DataError> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }

        let rows = sqlx::query_as::<_, DbTransformDraftRow>(&format!(
            r#"SELECT {DRAFT_ROW_COLUMNS} FROM transform_draft WHERE transform_id = ANY($1)"#,
        ))
        .bind(ids)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(DbTransformDraft::from).collect())
    }

    async fn delete_transform_draft(&self, id: domain::db::db_transform_draft::TransformDraftId) -> Result<(), DataError> {
        // Today a draft and its transform share the same underlying id (see
        // the trait doc comment), so this guards and cascades identically to
        // `delete_transform`.
        self.delete_transform(id).await
    }

    async fn save_transform_draft(
        &self,
        id: domain::db::db_transform_draft::TransformDraftId,
        source_code: String,
        resource_id: Option<ResourceId>,
    ) -> Result<DbTransformDraft, DataError> {
        let mut tx = self.pool.begin().await?;

        sqlx::query(r#"UPDATE transform_draft SET source_code = $2, updated_at = now() WHERE transform_id = $1"#)
            .bind(id)
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
            let result = sqlx::query(
                r#"
                UPDATE transform_draft d
                SET wasm_bytecode = r.wasm_bytecode,
                    wasm_source_code = $3,
                    name = r.name,
                    description = r.description,
                    metadata = r.metadata,
                    updated_at = now()
                FROM transform_resource r
                JOIN transform_ticket t ON t.ticket_id = r.ticket_id
                WHERE d.transform_id = $1
                  AND r.resource_id = $2
                  AND t.transform_id = $1
                  AND t.source_code = $3
                "#,
            )
            .bind(id)
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

        let row = sqlx::query_as::<_, DbTransformDraftRow>(&format!(
            r#"SELECT {DRAFT_ROW_COLUMNS} FROM transform_draft WHERE transform_id = $1"#,
        ))
        .bind(id)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(row.into())
    }

    async fn publish_compiled_transform(
        &self,
        id: domain::db::db_transform_draft::TransformDraftId,
        wasm_bytecode: Vec<u8>,
        source_code: String,
        name: String,
        description: Option<String>,
        metadata: String,
    ) -> Result<DbTransform, DataError> {
        let row = sqlx::query_as::<_, DbTransformRow>(&format!(
            r#"
            UPDATE transform
            SET name = $2, description = $3, source_code = $4, wasm_bytecode = $5, metadata = $6
            WHERE transform_id = $1
            RETURNING {TRANSFORM_ROW_COLUMNS_BARE}
            "#,
        ))
        .bind(id)
        .bind(&name)
        .bind(&description)
        .bind(&source_code)
        .bind(&wasm_bytecode)
        .bind(&metadata)
        .fetch_one(&self.pool)
        .await?;

        Ok(row.into())
    }
}
