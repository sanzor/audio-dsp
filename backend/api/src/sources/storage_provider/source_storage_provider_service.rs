use domain::{db::db_source::SourceId, sources::source_bundle::SourcePayload};
use sqlx::PgPool;

use super::source_storage_provider::SourceStorageProvider;

/// Backed directly by a `sources JOIN source_storage` query (the same join shape as
/// `PostgresStoredTracksDataProvider`), collapsed into a single provider/service pair
/// rather than tracks' two-layer split -- sources has none of the Region/RegionSet
/// cascade complexity that motivated separating tracks' storage lookup into its own
/// `stored_tracks` module.
pub struct SourceStorageProviderService {
    pool: PgPool,
}

impl SourceStorageProviderService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl SourceStorageProvider for SourceStorageProviderService {
    async fn get_source_payload(&self, source_id: &SourceId) -> Result<SourcePayload, String> {
        let canonical_audio: Vec<u8> = sqlx::query_scalar(
            "SELECT data FROM source_storage WHERE source_id = $1",
        )
        .bind(source_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(SourcePayload { canonical_audio })
    }

    async fn insert_source_payload(&self, source_id: &SourceId, payload: SourcePayload) -> Result<(), String> {
        sqlx::query("INSERT INTO source_storage (source_id, data) VALUES ($1, $2)")
            .bind(source_id)
            .bind(payload.canonical_audio)
            .execute(&self.pool)
            .await
            .map(|_| ())
            .map_err(|e| e.to_string())
    }
}
