use async_trait::async_trait;
use scoring_domain::client_tier::ClientTier;
use sqlx::{FromRow, PgPool};
use std::str::FromStr;
use tracing::error;

use crate::api_keys::update_api_key_params::UpdateApiKeyParams;
use crate::{
    api_keys::data_provider::api_keys_data_provider::ApiKeysDataProvider,
    domain::db::{
        db_api_key::{ApiKeyId, DbApiKey},
        db_organization::OrganizationId,
        db_user::UserId,
    },
};

pub struct ApiKeysDataProviderService {
    pool: PgPool,
}

impl ApiKeysDataProviderService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[derive(Debug, FromRow)]
struct DbOrgMeta {
    tier: String,
    status: String,
}

#[async_trait]
impl ApiKeysDataProvider for ApiKeysDataProviderService {
    async fn create_api_key(
        &self,
        org_id: OrganizationId,
        key: String,
        label: Option<String>,
        created_by: Option<UserId>,
    ) -> Result<DbApiKey, String> {
        let created_by_i64: Option<i64> = created_by
            .map(|value| i64::try_from(value).map_err(|_| "created_by out of range".to_string()))
            .transpose()?;
        let record = sqlx::query_as::<_, DbApiKey>(
            "INSERT INTO api_keys (org_id, \"key\", label, is_active, created_by) \
             VALUES ($1, $2, $3, TRUE, $4) \
             RETURNING id, org_id, \"key\", label, is_active, created_by",
        )
        .bind(org_id)
        .bind(key)
        .bind(label)
        .bind(created_by_i64)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            error!(error = %e, "create api key failed");
            e.to_string()
        })?;

        Ok(record)
    }

    async fn update_api_key(
        &self,
        api_key_id: ApiKeyId,
        params: UpdateApiKeyParams,
    ) -> Result<Option<DbApiKey>, String> {
        let api_key_id =
            i64::try_from(api_key_id).map_err(|_| "api key id out of range".to_string())?;
        let record = sqlx::query_as::<_, DbApiKey>(
            "UPDATE api_keys \
             SET label = COALESCE($2, label), is_active = COALESCE($3, is_active) \
             WHERE id = $1 \
             RETURNING id, org_id, \"key\", label, is_active, created_by",
        )
        .bind(api_key_id)
        .bind(params.label)
        .bind(params.is_active)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            error!(error = %e, "update api key failed");
            e.to_string()
        })?;

        Ok(record)
    }

    async fn delete_api_key(
        &self,
        api_key_id: ApiKeyId,
    ) -> Result<Option<(OrganizationId, String)>, String> {
        let api_key_id =
            i64::try_from(api_key_id).map_err(|_| "api key id out of range".to_string())?;
        let record = sqlx::query_as::<_, DbApiKey>(
            "DELETE FROM api_keys WHERE id = $1 RETURNING id, org_id, \"key\", label, is_active, created_by",
        )
        .bind(api_key_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            error!(error = %e, "delete api key failed");
            e.to_string()
        })?;

        Ok(record.map(|row| (row.org_id, row.key)))
    }

    async fn get_api_key(&self, api_key_id: ApiKeyId) -> Result<Option<DbApiKey>, String> {
        let api_key_id =
            i64::try_from(api_key_id).map_err(|_| "api key id out of range".to_string())?;
        let record = sqlx::query_as::<_, DbApiKey>(
            "SELECT id, org_id, \"key\", label, is_active, created_by \
             FROM api_keys WHERE id = $1",
        )
        .bind(api_key_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            error!(error = %e, "get api key failed");
            e.to_string()
        })?;

        Ok(record)
    }

    async fn get_all_api_keys(&self) -> Result<Vec<DbApiKey>, String> {
        let records = sqlx::query_as::<_, DbApiKey>(
            "SELECT id, org_id, \"key\", label, is_active, created_by \
             FROM api_keys ORDER BY id",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            error!(error = %e, "get all api keys failed");
            e.to_string()
        })?;

        Ok(records)
    }

    async fn list_api_keys_by_org(&self, org_id: OrganizationId) -> Result<Vec<DbApiKey>, String> {
        let org_id = i64::try_from(org_id).map_err(|_| "org id out of range".to_string())?;
        let records = sqlx::query_as::<_, DbApiKey>(
            "SELECT id, org_id, \"key\", label, is_active, created_by \
             FROM api_keys WHERE org_id = $1 ORDER BY id",
        )
        .bind(org_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            error!(error = %e, "list api keys by org failed");
            e.to_string()
        })?;

        Ok(records)
    }

    async fn get_org_metadata(&self, org_id: OrganizationId) -> Result<(ClientTier, bool), String> {
        let org_id = i64::try_from(org_id).map_err(|_| "org id out of range".to_string())?;
        let record = sqlx::query_as::<_, DbOrgMeta>(
            "SELECT tier, status FROM subscriptions WHERE org_id = $1",
        )
        .bind(org_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        let tier = record
            .as_ref()
            .and_then(|meta| ClientTier::from_str(&meta.tier).ok())
            .unwrap_or(ClientTier::Free);
        let status = record
            .as_ref()
            .map(|meta| meta.status.as_str())
            .unwrap_or("active");
        Ok((tier, status == "active"))
    }
}
