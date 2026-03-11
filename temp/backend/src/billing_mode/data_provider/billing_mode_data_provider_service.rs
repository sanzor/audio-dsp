use async_trait::async_trait;
use sqlx::PgPool;
use tracing::error;

use crate::billing_mode::data_provider::billing_mode_data_provider::BillingModeDataProvider;
use crate::domain::db::db_billing_mode::DbBillingModeConfig;
use crate::domain::db::db_organization::OrganizationId;

pub struct BillingModeDataProviderService {
    pool: PgPool,
}

impl BillingModeDataProviderService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl BillingModeDataProvider for BillingModeDataProviderService {
    async fn get_mode(
        &self,
        org_id: OrganizationId,
    ) -> Result<Option<DbBillingModeConfig>, String> {
        let org_id = i64::try_from(org_id).map_err(|_| "org id out of range".to_string())?;
        sqlx::query_as::<_, DbBillingModeConfig>(
            "SELECT org_id, mode, updated_at FROM billing_mode_config WHERE org_id = $1",
        )
        .bind(org_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            error!(error = %e, "get billing mode failed");
            e.to_string()
        })
    }

    async fn upsert_mode(
        &self,
        org_id: OrganizationId,
        mode: &str,
    ) -> Result<DbBillingModeConfig, String> {
        let org_id = i64::try_from(org_id).map_err(|_| "org id out of range".to_string())?;
        sqlx::query_as::<_, DbBillingModeConfig>(
            "INSERT INTO billing_mode_config (org_id, mode) \
             VALUES ($1, $2) \
             ON CONFLICT (org_id) DO UPDATE SET mode = EXCLUDED.mode, updated_at = NOW() \
             RETURNING org_id, mode, updated_at",
        )
        .bind(org_id)
        .bind(mode)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            error!(error = %e, "upsert billing mode failed");
            e.to_string()
        })
    }
}
