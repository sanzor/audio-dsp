use async_trait::async_trait;
use domain::{
    db::{
        db_region::{DbRegion, RegionId},
        db_region_set::{DbRegionSet, RegionSetId},
        db_track::TrackId,
    },
    region_set::{
        copy_region_set_params::CopyRegionSetParams,
        create_region_set_params::CreateRegionSetParams,
        edit_region_set_params::EditRegionSetParams,
    },
    regions::{
        add_region_params::AddRegionParams,
        copy_region_params::CopyRegionParams,
        delete_region_params::DeleteRegionParams,
        edit_region_params::EditRegionParams,
    },
};
use sqlx::PgPool;
use ulid::Ulid;

use super::region_sets_provider::RegionSetsProvider;

pub struct PostgresRegionSetsProvider {
    pool: PgPool,
}

impl PostgresRegionSetsProvider {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RegionSetsProvider for PostgresRegionSetsProvider {
    async fn create_region_set(&self, params: CreateRegionSetParams) -> Result<DbRegionSet, String> {
        let id = Ulid::new().to_string();
        sqlx::query_as::<_, DbRegionSet>(
            "INSERT INTO region_sets (region_set_id, track_id, name, track_length_seconds)
             VALUES ($1, $2, $3, $4)
             RETURNING region_set_id, track_id, name, track_length_seconds, created_at"
        )
        .bind(&id)
        .bind(&params.track_id)
        .bind(params.name.unwrap_or_else(|| "set".into()))
        .bind(params.track_length)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn get_region_set(&self, set_id: &RegionSetId) -> Result<DbRegionSet, String> {
        sqlx::query_as::<_, DbRegionSet>(
            "SELECT region_set_id, track_id, name, track_length_seconds, created_at FROM region_sets WHERE region_set_id = $1"
        )
        .bind(set_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn get_region_sets_for_track(&self, track_id: &TrackId) -> Result<Vec<DbRegionSet>, String> {
        sqlx::query_as::<_, DbRegionSet>(
            "SELECT region_set_id, track_id, name, track_length_seconds, created_at FROM region_sets WHERE track_id = $1"
        )
        .bind(track_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn get_region_sets(&self) -> Result<Vec<DbRegionSet>, String> {
        sqlx::query_as::<_, DbRegionSet>(
            "SELECT region_set_id, track_id, name, track_length_seconds, created_at FROM region_sets"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn edit_region_set(&self, params: EditRegionSetParams) -> Result<DbRegionSet, String> {
        sqlx::query_as::<_, DbRegionSet>(
            "UPDATE region_sets SET name = COALESCE($1, name)
             WHERE region_set_id = $2
             RETURNING region_set_id, track_id, name, track_length_seconds, created_at"
        )
        .bind(params.name)
        .bind(&params.region_set_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn delete_region_set(&self, set_id: &RegionSetId) -> Result<(), String> {
        sqlx::query("DELETE FROM region_sets WHERE region_set_id = $1")
            .bind(set_id)
            .execute(&self.pool)
            .await
            .map(|_| ())
            .map_err(|e| e.to_string())
    }

    async fn copy_region_set(&self, params: CopyRegionSetParams) -> Result<DbRegionSet, String> {
        let new_id = Ulid::new().to_string();
        sqlx::query_as::<_, DbRegionSet>(
            "INSERT INTO region_sets (region_set_id, track_id, name, track_length_seconds)
             SELECT $1, track_id, $2, track_length_seconds FROM region_sets WHERE region_set_id = $3
             RETURNING region_set_id, track_id, name, track_length_seconds, created_at"
        )
        .bind(&new_id)
        .bind(&params.region_set_name)
        .bind(&params.region_set_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn get_regions_for_region_set(&self, set_id: &RegionSetId) -> Result<Vec<DbRegion>, String> {
        sqlx::query_as::<_, DbRegion>(
            "SELECT region_id, region_set_id, name, start_time_seconds, end_time_seconds, created_at FROM regions WHERE region_set_id = $1"
        )
        .bind(set_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn add_region(&self, params: AddRegionParams) -> Result<DbRegion, String> {
        let region_id = Ulid::new().to_string();
        sqlx::query_as::<_, DbRegion>(
            "INSERT INTO regions (region_id, region_set_id, name, start_time_seconds, end_time_seconds)
             VALUES ($1, $2, $3, $4, $5)
             RETURNING region_id, region_set_id, name, start_time_seconds, end_time_seconds, created_at"
        )
        .bind(&region_id)
        .bind(&params.region_set_id)
        .bind(&params.name)
        .bind(params.start_time)
        .bind(params.end_time)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn edit_region(&self, params: EditRegionParams) -> Result<DbRegion, String> {
        sqlx::query_as::<_, DbRegion>(
            "UPDATE regions SET
             name = COALESCE($1, name),
             start_time_seconds = COALESCE($2, start_time_seconds),
             end_time_seconds = COALESCE($3, end_time_seconds)
             WHERE region_id = $4
             RETURNING region_id, region_set_id, name, start_time_seconds, end_time_seconds, created_at"
        )
        .bind(params.name)
        .bind(params.start_time)
        .bind(params.end_time)
        .bind(&params.region_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn delete_region(&self, params: DeleteRegionParams) -> Result<(), String> {
        sqlx::query("DELETE FROM regions WHERE region_id = $1")
            .bind(&params.region_id)
            .execute(&self.pool)
            .await
            .map(|_| ())
            .map_err(|e| e.to_string())
    }

    async fn copy_region(&self, params: CopyRegionParams) -> Result<DbRegion, String> {
        let new_id = Ulid::new().to_string();
        sqlx::query_as::<_, DbRegion>(
            "INSERT INTO regions (region_id, region_set_id, name, start_time_seconds, end_time_seconds)
             SELECT $1, $2, $3, start_time_seconds, end_time_seconds FROM regions WHERE region_id = $4
             RETURNING region_id, region_set_id, name, start_time_seconds, end_time_seconds, created_at"
        )
        .bind(&new_id)
        .bind(&params.destination_region_set_id)
        .bind(&params.region_copy_name)
        .bind(&params.source_region_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn get_region(&self, region_id: &RegionId) -> Result<DbRegion, String> {
        sqlx::query_as::<_, DbRegion>(
            "SELECT region_id, region_set_id, name, start_time_seconds, end_time_seconds, created_at FROM regions WHERE region_id = $1"
        )
        .bind(region_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }
}
