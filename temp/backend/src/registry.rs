use deadpool_redis::redis::aio::MultiplexedConnection;
use deadpool_redis::redis::AsyncCommands;
use rust_shared::organization_id::OrganizationId;
use scoring_domain::client_tier::ClientTier;
use scoring_domain::org_metadata::OrganizationMetadata;
use sqlx::PgPool;
use tracing::error;

use common::constants::API_KEYS_REGISTRY_KEY;

pub async fn update_api_key_registry_for_org(
    pool: &PgPool,
    redis: &MultiplexedConnection,
    org_id: OrganizationId,
) -> Result<(), String> {
    let keys: Vec<String> =
        sqlx::query_scalar("SELECT \"key\" FROM api_keys WHERE org_id = $1 AND is_active = TRUE")
            .bind(org_id)
            .fetch_all(pool)
            .await
            .map_err(|e| {
                error!(error = %e, "failed to load api keys for registry");
                e.to_string()
            })?;

    let metadata_base = OrganizationMetadata {
        organization_id: org_id,
        tier: ClientTier::Free,
        is_active: true,
    };

    let mut conn = redis.clone();
    for key in keys {
        let payload = serde_json::to_string(&metadata_base).map_err(|e| e.to_string())?;

        conn.hset(API_KEYS_REGISTRY_KEY, key, payload)
            .await
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}
