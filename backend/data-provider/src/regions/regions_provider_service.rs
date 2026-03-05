use domain::{
    db::{
        db_region::{DbRegion, RegionId},
        db_region_set::RegionSetId,
    },
    regions::{
        add_region_params::AddRegionParams,
        copy_region_params::CopyRegionParams,
        delete_region_params::DeleteRegionParams,
        edit_region_params::EditRegionParams,
        region_subtree::RegionSubtree,
    },
};
use sqlx::PgPool;
use ulid::Ulid;

use super::regions_provider::RegionsProvider;

pub struct PostgresRegionsProvider {
    pool: PgPool,
}

impl PostgresRegionsProvider {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl RegionsProvider for PostgresRegionsProvider {
    async fn get_region(&self, region_id: &RegionId) -> Result<DbRegion, String> {
        sqlx::query_as::<_, DbRegion>(
            r#"
            SELECT region_id, region_set_id, name, start_time_seconds, end_time_seconds, created_at
            FROM regions
            WHERE region_id = $1
            "#,
        )
        .bind(region_id)
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
        let region_id: RegionId = Ulid::new().to_string();

        sqlx::query_as::<_, DbRegion>(
            r#"
            INSERT INTO regions (region_id, region_set_id, name, start_time_seconds, end_time_seconds)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING region_id, region_set_id, name, start_time_seconds, end_time_seconds, created_at
            "#,
        )
        .bind(region_id)
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
        let source = self.get_region(&params.source_region_id).await?;
        let new_region_id: RegionId = Ulid::new().to_string();

        sqlx::query_as::<_, DbRegion>(
            r#"
            INSERT INTO regions (region_id, region_set_id, name, start_time_seconds, end_time_seconds)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING region_id, region_set_id, name, start_time_seconds, end_time_seconds, created_at
            "#,
        )
        .bind(new_region_id)
        .bind(params.destination_region_set_id)
        .bind(params.region_copy_name)
        .bind(source.start_time_seconds)
        .bind(source.end_time_seconds)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn get_region_subtree(&self, region_id: &RegionId) -> Result<RegionSubtree, String> {
        let region = self.get_region(region_id).await?;
        Ok(RegionSubtree {
            region_id: region.region_id,
            region_set_id: region.region_set_id,
            name: region.name,
            start_time: region.start_time_seconds,
            end_time: region.end_time_seconds,
            graph: None, // graph internals are lazy-loaded via GraphsProvider
        })
    }
}
