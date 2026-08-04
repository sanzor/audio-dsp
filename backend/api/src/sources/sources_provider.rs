use domain::{
    db::{ProjectId, db_source::SourceId}, sources::{raw_source::RawSource, source_bundle::SourceBundle, source_meta::SourceMeta}, update_source_info_params::UpdateSourceInfoParams,
};

#[async_trait::async_trait]
pub trait SourcesProvider: Send + Sync {
    async fn get_source_meta(&self, source_id: &SourceId) -> Result<SourceMeta, String>;
    async fn get_source(&self, source_id: &SourceId) -> Result<SourceBundle, String>;
    async fn get_all_source_metas(&self) -> Result<Vec<SourceMeta>, String>;
    async fn insert_source(&self, source: RawSource, project_id: ProjectId) -> Result<SourceBundle, String>;
    async fn delete_source(&self, source_id: &SourceId) -> Result<(), String>;
    async fn update_source_info(
        &self,
        source_id: &SourceId,
        params: UpdateSourceInfoParams,
    ) -> Result<SourceMeta, String>;
}
