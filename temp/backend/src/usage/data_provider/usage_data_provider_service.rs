use async_trait::async_trait;
use sqlx::PgPool;
use tracing::error;

use crate::domain::data_error::DataError;
use crate::domain::db::db_organization::OrganizationId;
use crate::domain::db::db_token_bucket_usage::DbTokenBucketUsage;
use crate::domain::db::db_token_window_usage::DbTokenWindowUsage;
use crate::domain::db::usage_snapshot::UsageSnapshot;
use crate::jobs::usage_worker::usage_worker_processor::RateLimitUsageSnapshot;
use crate::usage::data_provider::usage_data_provider::UsageDataProvider;

pub struct UsageDataProviderService {
    pool: PgPool,
}

impl UsageDataProviderService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UsageDataProvider for UsageDataProviderService {

    async fn upsert_subscription_usage(
        &self,
        org_id: OrganizationId,
        snapshot: RateLimitUsageSnapshot,
    ) -> Result<(), String> {
        match snapshot {
            RateLimitUsageSnapshot::TokenBucket { limit, bucket_tokens } => {
                sqlx::query(
                    "INSERT INTO token_bucket_usage (org_id, \"limit\", bucket_tokens, updated_at) \
                     VALUES ($1, $2, $3, NOW()) \
                     ON CONFLICT (org_id) DO UPDATE SET \
                       \"limit\" = EXCLUDED.\"limit\", \
                       bucket_tokens = EXCLUDED.bucket_tokens, \
                       updated_at = NOW()",
                )
                .bind(org_id)
                .bind(limit as i64)
                .bind(bucket_tokens)
                .execute(&self.pool)
                .await
                .map_err(|e| e.to_string())?;
            }
            RateLimitUsageSnapshot::TokenWindow {
                limit,
                window_size_secs,
                window_counter,
                window_start,
            } => {
                sqlx::query(
                    "INSERT INTO token_window_usage (org_id, \"limit\", window_size_secs, window_counter, window_start, updated_at) \
                     VALUES ($1, $2, $3, $4, $5, NOW()) \
                     ON CONFLICT (org_id) DO UPDATE SET \
                       \"limit\" = EXCLUDED.\"limit\", \
                       window_size_secs = EXCLUDED.window_size_secs, \
                       window_counter = EXCLUDED.window_counter, \
                       window_start = EXCLUDED.window_start, \
                       updated_at = NOW()",
                )
                .bind(org_id)
                .bind(limit as i64)
                .bind(window_size_secs as i64)
                .bind(window_counter)
                .bind(window_start)
                .execute(&self.pool)
                .await
                .map_err(|e| e.to_string())?;
            }
            RateLimitUsageSnapshot::Default => {}
        }

        Ok(())
    }


    async fn get_usage(
        &self,
        org_id: OrganizationId,
    ) -> Result<Option<UsageSnapshot>, DataError> {
        let bucket = sqlx::query_as::<_, DbTokenBucketUsage>(
            r#"SELECT org_id, "limit", bucket_tokens FROM token_bucket_usage WHERE org_id = $1"#,
        )
        .bind(org_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            error!(org_id, error = %e, "get usage from db failed");
            DataError::from(e)
        })?;

        if let Some(row) = bucket {
            return Ok(Some(UsageSnapshot::TokenBucket(row)));
        }

        let window = sqlx::query_as::<_, DbTokenWindowUsage>(
            r#"SELECT org_id, "limit", window_size_secs, window_counter, window_start
               FROM token_window_usage WHERE org_id = $1"#,
        )
        .bind(org_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            error!(org_id, error = %e, "get usage from db failed");
            DataError::from(e)
        })?;

        Ok(window.map(UsageSnapshot::TokenWindow))
    }

    async fn update_bucket_tokens(
        &self,
        org_id: OrganizationId,
        bucket_tokens: i64,
    ) -> Result<(), String> {
        sqlx::query(
            "UPDATE token_bucket_usage SET bucket_tokens = $2, updated_at = NOW() WHERE org_id = $1",
        )
        .bind(org_id)
        .bind(bucket_tokens)
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(())
    }
}
