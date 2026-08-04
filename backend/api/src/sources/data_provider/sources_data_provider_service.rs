use domain::{
    db::db_source::{DbSource, DbSourceMeta, SourceId},
    sources::source_info::SourceInfo,
    update_source_info_params::UpdateSourceInfoParams,
};
use sqlx::PgPool;

use super::sources_data_provider::SourcesDataProvider;

pub struct PostgresSourcesDataProvider {
    pool: PgPool,
}

impl PostgresSourcesDataProvider {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl SourcesDataProvider for PostgresSourcesDataProvider {
    async fn get_source(&self, source_id: &SourceId) -> Result<DbSource, String> {
        sqlx::query_as::<_, DbSource>(
            "SELECT source_id, name, extension, length_seconds, created_at FROM sources WHERE source_id = $1"
        )
        .bind(source_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn get_all_source_metas(&self) -> Result<Vec<DbSourceMeta>, String> {
        sqlx::query_as::<_, DbSourceMeta>(
            "SELECT source_id, name, extension, length_seconds, created_at FROM sources ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn delete_source(&self, source_id: &SourceId) -> Result<(), String> {
        sqlx::query("DELETE FROM sources WHERE source_id = $1")
            .bind(source_id)
            .execute(&self.pool)
            .await
            .map(|_| ())
            .map_err(|e| e.to_string())
    }

    async fn insert_source(&self, source_info: SourceInfo, project_id: i32) -> Result<DbSource, String> {
        sqlx::query_as::<_, DbSource>(
            "INSERT INTO sources (name, extension, length_seconds, project_id)
             VALUES ($1, $2, $3, $4)
             RETURNING source_id, name, extension, length_seconds, created_at",
        )
        .bind(&source_info.name)
        .bind(&source_info.extension)
        .bind(source_info.length)
        .bind(project_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn update_source_info(
        &self,
        source_id: &SourceId,
        params: UpdateSourceInfoParams,
    ) -> Result<DbSource, String> {
        sqlx::query_as::<_, DbSource>(
            "UPDATE sources SET name = $2 WHERE source_id = $1
             RETURNING source_id, name, extension, length_seconds, created_at",
        )
        .bind(source_id)
        .bind(params.source_name)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }
}
