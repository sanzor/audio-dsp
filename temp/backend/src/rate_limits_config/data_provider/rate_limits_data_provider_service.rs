use async_trait::async_trait;
use sqlx::{types::Json, PgPool};
use tracing::error;

use crate::domain::db::db_organization::OrganizationId;
use crate::domain::db::db_rate_limit_config::DbRateLimitConfig;
use crate::rate_limits_config::data_provider::rate_limits_data_provider::RateLimitsDataProvider;

pub struct RateLimitsDataProviderService {
    pool: PgPool,
}

impl RateLimitsDataProviderService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RateLimitsDataProvider for RateLimitsDataProviderService {
    async fn get_config(
        &self,
        org_id: OrganizationId,
    ) -> Result<Option<DbRateLimitConfig>, String> {
        let org_id = i64::try_from(org_id).map_err(|_| "org id out of range".to_string())?;
        let record = sqlx::query_as::<_, DbRateLimitConfig>(
            "SELECT org_id, config, updated_at FROM rate_limit_config WHERE org_id = $1",
        )
        .bind(org_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            error!(error = %e, "get rate limit config failed");
            e.to_string()
        })?;

        Ok(record)
    }

    async fn upsert_config(
        &self,
        org_id: OrganizationId,
        config: serde_json::Value,
    ) -> Result<DbRateLimitConfig, String> {
        let org_id = i64::try_from(org_id).map_err(|_| "org id out of range".to_string())?;
        let stored = Json(config);
        let record = sqlx::query_as::<_, DbRateLimitConfig>(
            "INSERT INTO rate_limit_config (org_id, config) \
             VALUES ($1, $2) \
             ON CONFLICT (org_id) DO UPDATE SET config = EXCLUDED.config, updated_at = NOW() \
             RETURNING org_id, config, updated_at",
        )
        .bind(org_id)
        .bind(stored)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            error!(error = %e, "upsert rate limit config failed");
            e.to_string()
        })?;

        Ok(record)
    }

    async fn create_event(
        &self,
        org_id: OrganizationId,
        event_type: &str,
        tokens: Option<i64>,
    ) -> Result<(), String> {
        let org_id = i64::try_from(org_id).map_err(|_| "org id out of range".to_string())?;
        sqlx::query(
            "INSERT INTO rate_limit_events (org_id, event_type, tokens) VALUES ($1, $2, $3)",
        )
        .bind(org_id)
        .bind(event_type)
        .bind(tokens)
        .execute(&self.pool)
        .await
        .map_err(|e| {
            error!(error = %e, "create rate limit event failed");
            e.to_string()
        })?;
        Ok(())
    }
}
