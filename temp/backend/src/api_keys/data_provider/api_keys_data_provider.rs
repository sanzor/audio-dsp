use async_trait::async_trait;
use scoring_domain::client_tier::ClientTier;

use crate::{
    api_keys::update_api_key_params::UpdateApiKeyParams,
    domain::db::{
        db_api_key::{ApiKeyId, DbApiKey},
        db_organization::OrganizationId,
        db_user::UserId,
    },
};

#[async_trait]
pub trait ApiKeysDataProvider: Send + Sync {
    async fn create_api_key(
        &self,
        org_id: OrganizationId,
        key: String,
        label: Option<String>,
        created_by: Option<UserId>,
    ) -> Result<DbApiKey, String>;
    async fn update_api_key(
        &self,
        api_key_id: ApiKeyId,
        params: UpdateApiKeyParams,
    ) -> Result<Option<DbApiKey>, String>;
    async fn delete_api_key(
        &self,
        api_key_id: ApiKeyId,
    ) -> Result<Option<(OrganizationId, String)>, String>;
    async fn get_api_key(&self, api_key_id: ApiKeyId) -> Result<Option<DbApiKey>, String>;
    async fn get_all_api_keys(&self) -> Result<Vec<DbApiKey>, String>;
    async fn list_api_keys_by_org(&self, org_id: OrganizationId) -> Result<Vec<DbApiKey>, String>;
    async fn get_org_metadata(&self, org_id: OrganizationId) -> Result<(ClientTier, bool), String>;
}
