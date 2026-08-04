use domain::{
    db::db_source::{DbSource, DbSourceMeta, SourceId},
    sources::source_info::SourceInfo,
    update_source_info_params::UpdateSourceInfoParams,
};

#[async_trait::async_trait]
pub trait SourcesDataProvider: Send + Sync {
    async fn get_source(&self, source_id: &SourceId) -> Result<DbSource, String>;
    async fn get_all_source_metas(&self) -> Result<Vec<DbSourceMeta>, String>;
    async fn delete_source(&self, source_id: &SourceId) -> Result<(), String>;
    async fn insert_source(&self, source_info: SourceInfo, project_id: i32) -> Result<DbSource, String>;
    async fn update_source_info(
        &self,
        source_id: &SourceId,
        params: UpdateSourceInfoParams,
    ) -> Result<DbSource, String>;
}
