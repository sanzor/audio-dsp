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
        add_region_params::AddRegionParams, copy_region_params::CopyRegionParams,
        delete_region_params::DeleteRegionParams, edit_region_params::EditRegionParams,
    },
};
use sqlx::PgPool;
use ulid::Ulid;

use crate::region_set_provider::RegionSetsProvider;

pub struct PostgresRegionSetsProvider {
    pool: PgPool,
}

impl PostgresRegionSetsProvider {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl RegionSetsProvider for PostgresRegionSetsProvider {
    async fn create_region_set(&self, params: CreateRegionSetParams) -> Result<DbRegionSet, String> {
        let region_set_id: RegionSetId = Ulid::new().to_string();
        let name = params.name.unwrap_or_else(|| Ulid::new().to_string());

        sqlx::query_as::<_, DbRegionSet>(
            r#"
            INSERT INTO region_sets (region_set_id, track_id, name, track_length_seconds)
            VALUES ($1, $2, $3, $4)
            RETURNING region_set_id, track_id, name, track_length_seconds, created_at
            "#,
        )
        .bind(region_set_id)
        .bind(params.track_id)
        .bind(name)
        .bind(params.track_length)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

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

    async fn get_region_sets_for_track(&self, track_id: &TrackId) -> Result<Vec<DbRegionSet>, String> {
        sqlx::query_as::<_, DbRegionSet>(
            r#"
            SELECT region_set_id, track_id, name, track_length_seconds, created_at
            FROM region_sets
            WHERE track_id = $1
            ORDER BY created_at DESC
            "#,
        )
        .bind(track_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn get_region_sets(&self) -> Result<Vec<DbRegionSet>, String> {
        sqlx::query_as::<_, DbRegionSet>(
            r#"
            SELECT region_set_id, track_id, name, track_length_seconds, created_at
            FROM region_sets
            ORDER BY created_at DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn edit_region_set(&self, params: EditRegionSetParams) -> Result<DbRegionSet, String> {
        let Some(name) = params.name else {
            return self.get_region_set(&params.region_set_id).await;
        };

        sqlx::query_as::<_, DbRegionSet>(
            r#"
            UPDATE region_sets
            SET name = $2
            WHERE region_set_id = $1
            RETURNING region_set_id, track_id, name, track_length_seconds, created_at
            "#,
        )
        .bind(params.region_set_id)
        .bind(name)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn delete_region_set(&self, set_id: &RegionSetId) -> Result<(), String> {
        sqlx::query(r#"DELETE FROM region_sets WHERE region_set_id = $1"#)
            .bind(set_id)
            .execute(&self.pool)
            .await
            .map(|_| ())
            .map_err(|e| e.to_string())
    }

    async fn copy_region_set(&self, params: CopyRegionSetParams) -> Result<DbRegionSet, String> {
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        let source_set = sqlx::query_as::<_, DbRegionSet>(
            r#"
            SELECT region_set_id, track_id, name, track_length_seconds, created_at
            FROM region_sets
            WHERE region_set_id = $1
            "#,
        )
        .bind(&params.region_set_id)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        let new_set_id: RegionSetId = Ulid::new().to_string();

        let new_set = sqlx::query_as::<_, DbRegionSet>(
            r#"
            INSERT INTO region_sets (region_set_id, track_id, name, track_length_seconds)
            VALUES ($1, $2, $3, $4)
            RETURNING region_set_id, track_id, name, track_length_seconds, created_at
            "#,
        )
        .bind(&new_set_id)
        .bind(&source_set.track_id)
        .bind(&params.region_set_name)
        .bind(source_set.track_length_seconds)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        let source_regions = sqlx::query_as::<_, DbRegion>(
            r#"
            SELECT region_id, region_set_id, name, start_time_seconds, end_time_seconds, created_at
            FROM regions
            WHERE region_set_id = $1
            ORDER BY start_time_seconds ASC
            "#,
        )
        .bind(&params.region_set_id)
        .fetch_all(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        for region in source_regions {
            let new_region_id: RegionId = Ulid::new().to_string();
            sqlx::query(
                r#"
                INSERT INTO regions (region_id, region_set_id, name, start_time_seconds, end_time_seconds)
                VALUES ($1, $2, $3, $4, $5)
                "#,
            )
            .bind(new_region_id)
            .bind(&new_set_id)
            .bind(region.name)
            .bind(region.start_time_seconds)
            .bind(region.end_time_seconds)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;
        }

        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(new_set)
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
}
