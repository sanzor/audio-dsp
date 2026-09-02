use domain::{db::db_source::SourceId, sources::source_bundle::SourcePayload};

#[async_trait::async_trait]
pub trait SourceStorageProvider: Send + Sync {
    async fn get_source_payload(&self, source_id: &SourceId) -> Result<SourcePayload, String>;
    async fn insert_source_payload(
        &self,
        source_id: &SourceId,
        payload: SourcePayload,
    ) -> Result<(), String>;
}
