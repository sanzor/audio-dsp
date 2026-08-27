use domain::{
    db::{
        db_transform::{DbTransform, TransformId},
        db_transform_draft::{DbTransformDraft, TransformDraftId},
    },
    domain_user::UserId,
};
use sqlx::PgPool;

use crate::domain::data_error::DataError;

use super::transform_drafts_data_provider::TransformDraftsDataProvider;

pub struct PostgresTransformDraftsDataProvider {
    pool: PgPool,
}

impl PostgresTransformDraftsDataProvider {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[derive(sqlx::FromRow)]
struct DbTransformDraftRow {
    transform_id: TransformDraftId,
    source_code: Option<String>,
    wasm_bytecode: Option<Vec<u8>>,
    wasm_source_code: Option<String>,
    name: Option<String>,
    description: Option<String>,
    kind: String,
    metadata: Option<String>,
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
            metadata: row.metadata,
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

const TRANSFORM_ROW_COLUMNS_BARE: &str = "transform_id, name, description, icon, kind, source_code, wasm_bytecode, metadata, owner_user_id, created_at";
const DRAFT_ROW_COLUMNS: &str = "transform_id, source_code, wasm_bytecode, wasm_source_code, name, description, kind, metadata";

#[async_trait::async_trait]
impl TransformDraftsDataProvider for PostgresTransformDraftsDataProvider {
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
               VALUES ($1, $2, $3, $4, $5, NULL, NULL, NULL)
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
            VALUES ($1, NULL, $2, $3, $4, NULL)
            RETURNING {DRAFT_ROW_COLUMNS}
            "#,
        ))
        .bind(TransformDraftId::from(transform_id))
        .bind(&name)
        .bind(&description)
        .bind(&kind)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(row.into())
    }

    async fn get_transform_draft(&self, id: TransformDraftId) -> Result<DbTransformDraft, DataError> {
        let row = sqlx::query_as::<_, DbTransformDraftRow>(&format!(
            r#"SELECT {DRAFT_ROW_COLUMNS} FROM transform_draft WHERE transform_id = $1"#,
        ))
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(row.into())
    }

    async fn get_transform_drafts(&self, ids: &[TransformDraftId]) -> Result<Vec<DbTransformDraft>, DataError> {
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

    async fn get_transform_owner(&self, id: TransformDraftId) -> Result<UserId, DataError> {
        sqlx::query_scalar::<_, UserId>("SELECT owner_user_id FROM transform WHERE transform_id = $1")
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .map_err(DataError::from)
    }

    async fn delete_transform_draft(&self, id: TransformDraftId) -> Result<(), DataError> {
        // Same guard and cascade as
        // `PostgresTransformsDataProvider::delete_transform` — kept as this
        // provider's own SQL (not a call through to that trait) so this
        // slice has no runtime dependency on the published-transform data
        // provider. See `TransformDraftId`'s doc comment for why a draft
        // and its transform share the same underlying row/guard.
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

    async fn get_published_transforms(&self, ids: &[TransformId]) -> Result<Vec<DbTransform>, DataError> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }

        let rows = sqlx::query_as::<_, DbTransformRow>(&format!(
            r#"SELECT {TRANSFORM_ROW_COLUMNS_BARE} FROM transform WHERE transform_id = ANY($1)"#,
        ))
        .bind(ids)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(DbTransform::from).collect())
    }

    async fn save_primitive_draft(
        &self,
        id: TransformDraftId,
        source_code: String,
    ) -> Result<DbTransformDraft, DataError> {
        self.save_draft_row(id, Some(source_code), None).await
    }

    async fn save_composite_draft(
        &self,
        id: TransformDraftId,
        graph_json: String,
    ) -> Result<DbTransformDraft, DataError> {
        self.save_draft_row(id, None, Some(graph_json)).await
    }

    async fn publish_composite_transform(
        &self,
        id: TransformDraftId,
        name: String,
        description: Option<String>,
        metadata: String,
    ) -> Result<DbTransform, DataError> {
        let row = sqlx::query_as::<_, DbTransformRow>(&format!(
            r#"
            UPDATE transform
            SET name = $2, description = $3, metadata = $4
            WHERE transform_id = $1
            RETURNING {TRANSFORM_ROW_COLUMNS_BARE}
            "#,
        ))
        .bind(id)
        .bind(&name)
        .bind(&description)
        .bind(&metadata)
        .fetch_one(&self.pool)
        .await?;

        Ok(row.into())
    }

    async fn publish_compiled_transform(
        &self,
        id: TransformDraftId,
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
impl PostgresTransformDraftsDataProvider {
    /// Shared UPDATE mechanism behind `save_primitive_draft`/
    /// `save_composite_draft` — one of `source_code`/`graph_json` is always
    /// `None` (the field the other kind owns), so COALESCE leaves it as
    /// whatever's already saved rather than clobbering it. Never touches
    /// `wasm_bytecode`/`wasm_source_code` — those are written by the ticket
    /// worker as soon as a compile succeeds, not by a save (see
    /// `TransformsDataProvider::cache_compiled_binary_on_draft`).
    async fn save_draft_row(
        &self,
        id: TransformDraftId,
        source_code: Option<String>,
        graph_json: Option<String>,
    ) -> Result<DbTransformDraft, DataError> {
        let row = sqlx::query_as::<_, DbTransformDraftRow>(&format!(
            r#"
            UPDATE transform_draft
            SET source_code = COALESCE($2, source_code),
                metadata = COALESCE($3, metadata),
                updated_at = now()
            WHERE transform_id = $1
            RETURNING {DRAFT_ROW_COLUMNS}
            "#,
        ))
        .bind(id)
        .bind(&source_code)
        .bind(&graph_json)
        .fetch_one(&self.pool)
        .await?;

        Ok(row.into())
    }
}
