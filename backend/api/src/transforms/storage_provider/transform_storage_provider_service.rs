use domain::db::db_transform::{DbTransformBinary, TransformId};
use sqlx::PgPool;

use super::transform_storage_provider::TransformStorageProvider;

pub struct DbTransformStorageProvider {
    pool: PgPool,
}

impl DbTransformStorageProvider {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl TransformStorageProvider for DbTransformStorageProvider {
    async fn write_transform_binary(&self, transform_id: TransformId, binary: &[u8]) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT INTO transform_binaries (transform_id, wasm_bytecode)
            VALUES ($1, $2)
            ON CONFLICT (transform_id)
            DO UPDATE SET wasm_bytecode = EXCLUDED.wasm_bytecode
            "#,
        )
        .bind(transform_id)
        .bind(binary)
        .execute(&self.pool)
        .await
        .map(|_| ())
        .map_err(|e| e.to_string())
    }
    async fn get_transform_binary(&self, transform_id: TransformId) -> Result<Vec<u8>, String> {
        let record = sqlx::query_scalar::<_, Vec<u8>>(
            r#"SELECT wasm_bytecode FROM transform_binaries WHERE transform_id = $1"#,
        )
        .bind(transform_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        record.ok_or_else(|| format!("WASM binary not found for transform {transform_id}"))
    }

    async fn get_transform_binaries(&self, transform_ids: &[TransformId]) -> Result<Vec<DbTransformBinary>, String> {
        if transform_ids.is_empty() {
            return Ok(Vec::new());
        }

        let rows = sqlx::query_as::<_, DbTransformBinary>(
            r#"
            SELECT tb.transform_id, tb.wasm_bytecode
            FROM unnest($1::bigint[]) WITH ORDINALITY AS requested(transform_id, ord)
            JOIN transform_binaries tb ON tb.transform_id = requested.transform_id
            ORDER BY requested.ord
            "#,
        )
        .bind(transform_ids)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(rows)
    }
}
