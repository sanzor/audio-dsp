use async_trait::async_trait;

use crate::api_keys::create_api_key_params::CreateApiKeyParams;
use crate::api_keys::update_api_key_params::UpdateApiKeyParams;
use crate::api_keys::{
    create_api_key_result::CreateApiKeyResult, get_api_keys_result::GetApiKeyResult,
    update_api_key_result::UpdateApiKeyResult,
};
use crate::domain::db::db_api_key::{ApiKeyId, DbApiKey};
use crate::domain::db::db_organization::OrganizationId;

#[async_trait]
pub trait ApiKeysProvider: Send + Sync {
    async fn create_api_key(
        &self,
        params: CreateApiKeyParams,
    ) -> Result<CreateApiKeyResult, String>;
    async fn update_api_key(
        &self,
        api_key_id: ApiKeyId,
        params: UpdateApiKeyParams,
    ) -> Result<Option<UpdateApiKeyResult>, String>;
    async fn delete_api_key(&self, api_key_id: ApiKeyId) -> Result<bool, String>;
    async fn get_api_key(&self, api_key_id: ApiKeyId) -> Result<Option<GetApiKeyResult>, String>;
    async fn get_all_api_keys(&self) -> Result<Vec<DbApiKey>, String>;
    async fn list_api_keys_by_org(&self, org_id: OrganizationId) -> Result<Vec<DbApiKey>, String>;
}
