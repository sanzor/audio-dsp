use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use async_trait::async_trait;
use deadpool_redis::redis::aio::MultiplexedConnection;
use deadpool_redis::redis::AsyncCommands;
use scoring_domain::client_tier::ClientTier;
use scoring_domain::org_metadata::OrganizationMetadata;
use sqlx::PgPool;

use common::constants::API_KEYS_REGISTRY_KEY;

use crate::domain::db::db_organization::OrganizationId;
use crate::jobs::sync_worker::{
    sync_worker_processor::SyncWorkerProcessor,
    sync_worker_processor_params::SyncWorkerProcessorParams,
};

pub struct SyncWorkerProcessorService {
    redis: MultiplexedConnection,
    db_conn: PgPool,
}

impl SyncWorkerProcessorService {
    pub fn new(params: SyncWorkerProcessorParams) -> Self {
        Self {
            redis: params.redis,
            db_conn: params.db_conn,
        }
    }

    async fn sync_rate_limit_configs(&self) -> Result<(), String> {
        let configs: Vec<(i64, serde_json::Value)> =
            sqlx::query_as("SELECT org_id, config FROM rate_limit_config")
                .fetch_all(&self.db_conn)
                .await
                .map_err(|e| e.to_string())?;

        let mut conn = self.redis.clone();
        for (org_id, config_json) in configs {
            let key = format!("organization:{}", org_id);
            let config_string = serde_json::to_string(&config_json).map_err(|e| e.to_string())?;
            conn.hset::<_, _, _, ()>(&key, "config", config_string)
                .await
                .map_err(|e| e.to_string())?;
        }

        Ok(())
    }

    async fn sync_api_keys_registry(&self) -> Result<(), String> {
        let rows: Vec<(String, i64, String, Option<String>)> = sqlx::query_as(
            "SELECT ak.key, ak.org_id, o.status, s.tier \
             FROM api_keys ak \
             JOIN organizations o ON o.id = ak.org_id \
             LEFT JOIN subscriptions s ON s.org_id = ak.org_id \
             WHERE ak.is_active = TRUE",
        )
        .fetch_all(&self.db_conn)
        .await
        .map_err(|e| e.to_string())?;

        let mut desired: HashMap<String, String> = HashMap::with_capacity(rows.len());

        for (api_key, org_id, org_status, maybe_tier) in rows {
            let tier_str = maybe_tier.as_deref().unwrap_or("free");
            let tier = ClientTier::from_str(tier_str).unwrap_or(ClientTier::Free);
            let is_active = org_status == "active";

            let metadata = OrganizationMetadata {
                organization_id: org_id as OrganizationId,
                tier,
                is_active,
            };

            let payload = serde_json::to_string(&metadata).map_err(|e| e.to_string())?;
            desired.insert(api_key, payload);
        }

        let mut conn = self.redis.clone();
        let existing: Vec<String> = conn
            .hkeys(API_KEYS_REGISTRY_KEY)
            .await
            .map_err(|e| e.to_string())?;

        let desired_keys: HashSet<String> = desired.keys().cloned().collect();
        let stale: Vec<String> = existing
            .into_iter()
            .filter(|k| !desired_keys.contains(k))
            .collect();

        if !stale.is_empty() {
            let _: usize = conn
                .hdel(API_KEYS_REGISTRY_KEY, stale)
                .await
                .map_err(|e| e.to_string())?;
        }

        for (api_key, payload) in desired {
            conn.hset::<_, _, _, ()>(API_KEYS_REGISTRY_KEY, api_key, payload)
                .await
                .map_err(|e| e.to_string())?;
        }

        Ok(())
    }
}

#[async_trait]
impl SyncWorkerProcessor for SyncWorkerProcessorService {
    async fn sync(&self) -> Result<(), String> {
        self.sync_rate_limit_configs().await?;
        self.sync_api_keys_registry().await?;
        Ok(())
    }
}
