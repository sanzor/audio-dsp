use async_trait::async_trait;
use sqlx::PgPool;
use tracing::error;
use crate::domain::data_error::DataError;
use crate::domain::db::db_usage::DbUsage;
use crate::usage::data_provider::usage_data_provider::UsageDataProvider;

const SELECT: &str = "SELECT id, user_id, project_count, total_track_count, total_storage_bytes, updated_at::text FROM usage";

pub struct UsageDataProviderService {
    pool: PgPool,
}
impl UsageDataProviderService {
    pub fn new(pool: PgPool) -> Self { Self { pool } }
}

#[async_trait]
impl UsageDataProvider for UsageDataProviderService {
    async fn get_usage(&self, user_id: &str) -> Result<Option<DbUsage>, DataError> {
        sqlx::query_as::<_, DbUsage>(&format!("{SELECT} WHERE user_id = $1"))
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| { error!(error = %e, "get usage failed"); DataError::from(e) })
    }

    async fn refresh_usage(&self, user_id: &str) -> Result<DbUsage, DataError> {
        // Upsert: recalculate from source tables
        sqlx::query_as::<_, DbUsage>(
            "INSERT INTO usage (user_id, project_count, total_track_count, total_storage_bytes, updated_at)
             SELECT $1,
               (SELECT COUNT(*) FROM memberships WHERE user_id = $1),
               (SELECT COUNT(*) FROM tracks t JOIN projects p ON t.project_id = p.id JOIN memberships m ON m.project_id = p.id WHERE m.user_id = $1),
               (SELECT COALESCE(SUM(t.size_bytes), 0) FROM tracks t JOIN projects p ON t.project_id = p.id JOIN memberships m ON m.project_id = p.id WHERE m.user_id = $1),
               NOW()
             ON CONFLICT (user_id) DO UPDATE
               SET project_count = EXCLUDED.project_count,
                   total_track_count = EXCLUDED.total_track_count,
                   total_storage_bytes = EXCLUDED.total_storage_bytes,
                   updated_at = NOW()
             RETURNING id, user_id, project_count, total_track_count, total_storage_bytes, updated_at::text"
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| { error!(error = %e, "refresh usage failed"); DataError::from(e) })
    }
}
