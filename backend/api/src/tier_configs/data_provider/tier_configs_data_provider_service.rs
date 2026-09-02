use crate::domain::data_error::DataError;
use crate::domain::db::db_tier_config::DbTierConfig;
use crate::domain::tier::Tier;
use crate::tier_configs::data_provider::tier_configs_data_provider::TierConfigsDataProvider;
use crate::tier_configs::update_tier_config_params::UpdateTierConfigParams;
use async_trait::async_trait;
use sqlx::PgPool;
use tracing::error;

const SELECT: &str = "SELECT id, tier, max_projects, max_tracks_per_project, max_storage_bytes, updated_at::text FROM tier_configs";

pub struct TierConfigsDataProviderService {
    pool: PgPool,
}
impl TierConfigsDataProviderService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TierConfigsDataProvider for TierConfigsDataProviderService {
    async fn get_tier_config(&self, tier: &Tier) -> Result<Option<DbTierConfig>, DataError> {
        sqlx::query_as::<_, DbTierConfig>(&format!("{SELECT} WHERE tier = $1"))
            .bind(tier)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| {
                error!(error = %e, "get tier config failed");
                DataError::from(e)
            })
    }

    async fn update_tier_config(
        &self,
        tier: &Tier,
        params: UpdateTierConfigParams,
    ) -> Result<Option<DbTierConfig>, DataError> {
        let existing = self.get_tier_config(tier).await?;
        if existing.is_none() {
            return Ok(None);
        }
        let existing = existing.unwrap();
        let max_projects = params.max_projects.unwrap_or(existing.max_projects);
        let max_tracks = params
            .max_tracks_per_project
            .unwrap_or(existing.max_tracks_per_project);
        let max_storage = params
            .max_storage_bytes
            .unwrap_or(existing.max_storage_bytes);
        sqlx::query_as::<_, DbTierConfig>(
            "UPDATE tier_configs SET max_projects=$1, max_tracks_per_project=$2, max_storage_bytes=$3, updated_at=NOW() \
             WHERE tier=$4 RETURNING id, tier, max_projects, max_tracks_per_project, max_storage_bytes, updated_at::text"
        )
        .bind(max_projects).bind(max_tracks).bind(max_storage).bind(tier)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| { error!(error = %e, "update tier config failed"); DataError::from(e) })
    }

    async fn list_tier_configs(&self) -> Result<Vec<DbTierConfig>, DataError> {
        sqlx::query_as::<_, DbTierConfig>(SELECT)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| {
                error!(error = %e, "list tier configs failed");
                DataError::from(e)
            })
    }
}
