use async_trait::async_trait;
use sha1::{Digest, Sha1};
use std::{
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};
use tracing::info;

use crate::api_keys::create_api_key_params::CreateApiKeyParams;
use crate::api_keys::update_api_key_params::UpdateApiKeyParams;
use crate::{
    api_keys::{
        api_keys_provider::ApiKeysProvider, create_api_key_result::CreateApiKeyResult,
        data_provider::api_keys_data_provider::ApiKeysDataProvider,
        get_api_keys_result::GetApiKeyResult, update_api_key_result::UpdateApiKeyResult,
    },
    domain::db::{
        db_api_key::{ApiKeyId, DbApiKey},
        db_organization::OrganizationId,
    },
};

pub struct ApiKeysService {
    data_provider: Arc<dyn ApiKeysDataProvider>,
}

impl ApiKeysService {
    pub fn new(data_provider: Arc<dyn ApiKeysDataProvider>) -> Self {
        Self { data_provider }
    }

    fn generate_api_key(org_id: OrganizationId) -> String {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        let mut hasher = Sha1::new();
        hasher.update(org_id.to_le_bytes());
        hasher.update(now.to_le_bytes());
        hasher.update(format!("{:?}", std::thread::current().id()).as_bytes());
        format!("{:x}", hasher.finalize())
    }
}

#[async_trait]
impl ApiKeysProvider for ApiKeysService {
    async fn create_api_key(
        &self,
        params: CreateApiKeyParams,
    ) -> Result<CreateApiKeyResult, String> {
        info!(org_id = params.org_id, label = ?params.label, "create api key requested");
        let key = Self::generate_api_key(params.org_id);
        let record = self
            .data_provider
            .create_api_key(params.org_id, key, params.label, params.created_by)
            .await?;

        Ok(CreateApiKeyResult { api_key: record })
    }

    async fn update_api_key(
        &self,
        api_key_id: ApiKeyId,
        params: UpdateApiKeyParams,
    ) -> Result<Option<UpdateApiKeyResult>, String> {
        let record = self
            .data_provider
            .update_api_key(api_key_id, params)
            .await?;
        let Some(record) = record else {
            return Ok(None);
        };

        Ok(Some(UpdateApiKeyResult { api_key: record }))
    }

    async fn delete_api_key(&self, api_key_id: ApiKeyId) -> Result<bool, String> {
        info!(api_key_id, "delete api key requested");
        let record = self.data_provider.delete_api_key(api_key_id).await?;
        Ok(record.is_some())
    }

    async fn get_api_key(&self, api_key_id: ApiKeyId) -> Result<Option<GetApiKeyResult>, String> {
        info!(api_key_id, "get api key requested");
        let record = self.data_provider.get_api_key(api_key_id).await?;

        match record {
            Some(r) => Ok(Some(GetApiKeyResult { api_key: r })),
            None => Ok(None),
        }
    }

    async fn get_all_api_keys(&self) -> Result<Vec<DbApiKey>, String> {
        info!("get all api keys requested");
        let records = self.data_provider.get_all_api_keys().await?;
        Ok(records)
    }

    async fn list_api_keys_by_org(&self, org_id: OrganizationId) -> Result<Vec<DbApiKey>, String> {
        info!(org_id, "list api keys by org requested");
        let records = self.data_provider.list_api_keys_by_org(org_id).await?;
        Ok(records)
    }
}
