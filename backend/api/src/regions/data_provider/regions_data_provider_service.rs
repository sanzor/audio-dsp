use domain::{
    db::{
        db_region::{DbRegion, RegionId},
        db_region_set::{DbRegionSet, RegionSetId},
    },
    regions::{
        add_region_params::AddRegionParams,
        copy_region_params::CopyRegionParams,
        delete_region_params::DeleteRegionParams,
        edit_region_params::EditRegionParams,
    },
};
use sqlx::PgPool;

use super::regions_data_provider::RegionsDataProvider;

pub struct PostgresRegionsDataProvider {
    pool: PgPool,
}

impl PostgresRegionsDataProvider {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl RegionsDataProvider for PostgresRegionsDataProvider {
    async fn get_region_set(&self, set_id: &RegionSetId) -> Result<DbRegionSet, String> {
        sqlx::query_as::<_, DbRegionSet>(
            r#"
            SELECT region_set_id, track_id, name, track_length_seconds, created_at
            FROM region_sets
            WHERE region_set_id = $1
            "#,
        )
        .bind(set_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn get_regions_for_region_set(&self, set_id: &RegionSetId) -> Result<Vec<DbRegion>, String> {
        sqlx::query_as::<_, DbRegion>(
            r#"
            SELECT region_id, region_set_id, name, start_time_seconds, end_time_seconds, created_at
            FROM regions
            WHERE region_set_id = $1
            ORDER BY start_time_seconds ASC
            "#,
        )
        .bind(set_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn add_region(&self, params: AddRegionParams) -> Result<DbRegion, String> {
        sqlx::query_as::<_, DbRegion>(
            r#"
            INSERT INTO regions (region_set_id, name, start_time_seconds, end_time_seconds)
            VALUES ($1, $2, $3, $4)
            RETURNING region_id, region_set_id, name, start_time_seconds, end_time_seconds, created_at
            "#,
        )
        .bind(params.region_set_id)
        .bind(params.name)
        .bind(params.start_time)
        .bind(params.end_time)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn edit_region(&self, params: EditRegionParams) -> Result<DbRegion, String> {
        sqlx::query_as::<_, DbRegion>(
            r#"
            UPDATE regions
            SET
              name = COALESCE($3, name),
              start_time_seconds = COALESCE($4, start_time_seconds),
              end_time_seconds = COALESCE($5, end_time_seconds)
            WHERE region_id = $1 AND region_set_id = $2
            RETURNING region_id, region_set_id, name, start_time_seconds, end_time_seconds, created_at
            "#,
        )
        .bind(params.region_id)
        .bind(params.region_set_id)
        .bind(params.name)
        .bind(params.start_time)
        .bind(params.end_time)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn delete_region(&self, params: DeleteRegionParams) -> Result<(), String> {
        sqlx::query(r#"DELETE FROM regions WHERE region_id = $1 AND region_set_id = $2"#)
            .bind(params.region_id)
            .bind(params.region_set_id)
            .execute(&self.pool)
            .await
            .map(|_| ())
            .map_err(|e| e.to_string())
    }

    async fn copy_region(&self, params: CopyRegionParams) -> Result<DbRegion, String> {
        let source = sqlx::query_as::<_, DbRegion>(
            r#"
            SELECT region_id, region_set_id, name, start_time_seconds, end_time_seconds, created_at
            FROM regions
            WHERE region_id = $1 AND region_set_id = $2
            "#,
        )
        .bind(&params.source_region_id)
        .bind(&params.source_region_set_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        sqlx::query_as::<_, DbRegion>(
            r#"
            INSERT INTO regions (region_set_id, name, start_time_seconds, end_time_seconds)
            VALUES ($1, $2, $3, $4)
            RETURNING region_id, region_set_id, name, start_time_seconds, end_time_seconds, created_at
            "#,
        )
        .bind(params.destination_region_set_id)
        .bind(params.region_copy_name)
        .bind(source.start_time_seconds)
        .bind(source.end_time_seconds)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }
}
