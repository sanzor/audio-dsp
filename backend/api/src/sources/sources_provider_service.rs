use std::sync::Arc;

use audiolib::utils::encode_audio_buffer_as_wav;
use domain::{
    db::db_source::{DbSource, DbSourceMeta, SourceId},
    sources::{
        raw_source::{RawSource, SourceInfo},
        source_bundle::{SourceBundle, SourcePayload},
        source_meta::SourceMeta,
    },
    update_source_info_params::UpdateSourceInfoParams,
};

use super::{
    data_provider::sources_data_provider::SourcesDataProvider, sources_provider::SourcesProvider,
    storage_provider::source_storage_provider::SourceStorageProvider,
};

pub struct SourcesProviderService {
    data: Arc<dyn SourcesDataProvider>,
    storage: Arc<dyn SourceStorageProvider>,
}

impl SourcesProviderService {
    pub fn new(
        data: Arc<dyn SourcesDataProvider>,
        storage: Arc<dyn SourceStorageProvider>,
    ) -> Self {
        Self { data, storage }
    }

    fn to_meta(source: &DbSource) -> SourceMeta {
        SourceMeta {
            source_info: SourceInfo {
                name: source.name.clone(),
                extension: source.extension.clone(),
                length: source.length_seconds,
            },
            source_id: source.source_id,
        }
    }

    fn meta_from_db(source: DbSourceMeta) -> SourceMeta {
        SourceMeta {
            source_info: SourceInfo {
                name: source.name,
                extension: source.extension,
                length: source.length_seconds,
            },
            source_id: source.source_id,
        }
    }
}

#[async_trait::async_trait]
impl SourcesProvider for SourcesProviderService {
    async fn get_source_meta(&self, source_id: &SourceId) -> Result<SourceMeta, String> {
        let source = self.data.get_source(source_id).await?;
        Ok(Self::to_meta(&source))
    }

    async fn get_source(&self, source_id: &SourceId) -> Result<SourceBundle, String> {
        let payload = self.storage.get_source_payload(source_id).await?;
        let source = self.data.get_source(source_id).await?;
        let meta = Self::to_meta(&source);
        Ok(SourceBundle { payload, meta })
    }

    async fn get_all_source_metas(&self) -> Result<Vec<SourceMeta>, String> {
        let sources = self.data.get_all_source_metas().await?;
        Ok(sources.into_iter().map(Self::meta_from_db).collect())
    }

    async fn insert_source(
        &self,
        source: RawSource,
        workspace_id: i32,
    ) -> Result<SourceBundle, String> {
        let canonical_audio = encode_audio_buffer_as_wav(&source.data)
            .map_err(|_| "Could not encode source as wav".to_string())?;

        let db_source = self.data.insert_source(source.info, workspace_id).await?;
        let meta = Self::to_meta(&db_source);
        let payload = SourcePayload { canonical_audio };

        if let Err(err) = self
            .storage
            .insert_source_payload(&meta.source_id, payload.clone())
            .await
        {
            let _ = self.data.delete_source(&meta.source_id).await;
            return Err(err);
        }

        Ok(SourceBundle { meta, payload })
    }

    async fn delete_source(&self, source_id: &SourceId) -> Result<(), String> {
        self.data.delete_source(source_id).await
    }

    async fn update_source_info(
        &self,
        source_id: &SourceId,
        params: UpdateSourceInfoParams,
    ) -> Result<SourceMeta, String> {
        let db_source = self.data.update_source_info(source_id, params).await?;
        Ok(Self::to_meta(&db_source))
    }
}
