use domain::{
    db::{
        db_region::{DbRegion, RegionId},
        db_region_set::{DbRegionSet, RegionSetId},
        db_track::{DbTrack, TrackId},
    },
    region_set::{
        copy_region_set_params::CopyRegionSetParams,
        create_region_set_params::CreateRegionSetParams,
        edit_region_set_params::EditRegionSetParams,
    },
};
use sqlx::PgPool;

use super::region_sets_data_provider::RegionSetsDataProvider;

pub struct PostgresRegionSetsDataProvider {
    pool: PgPool,
}

impl PostgresRegionSetsDataProvider {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl RegionSetsDataProvider for PostgresRegionSetsDataProvider {
    async fn get_track(&self, track_id: &TrackId) -> Result<DbTrack, String> {
        sqlx::query_as::<_, DbTrack>(
            r#"
            SELECT track_id, name, extension, length_seconds, canonical_audio, created_at
            FROM tracks
            WHERE track_id = $1
            "#,
        )
        .bind(track_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn create_region_set(&self, params: CreateRegionSetParams) -> Result<DbRegionSet, String> {
        let name = params.name.unwrap_or_else(|| "New Region Set".to_string());

        sqlx::query_as::<_, DbRegionSet>(
            r#"
            INSERT INTO region_sets (track_id, name, track_length_seconds)
            VALUES ($1, $2, $3)
            RETURNING region_set_id, track_id, name, track_length_seconds, created_at
            "#,
        )
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

    async fn get_region_sets_for_track(
        &self,
        track_id: &TrackId,
    ) -> Result<Vec<DbRegionSet>, String> {
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

        let new_set = sqlx::query_as::<_, DbRegionSet>(
            r#"
            INSERT INTO region_sets (track_id, name, track_length_seconds)
            VALUES ($1, $2, $3)
            RETURNING region_set_id, track_id, name, track_length_seconds, created_at
            "#,
        )
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
            sqlx::query(
                r#"
                INSERT INTO regions (region_set_id, name, start_time_seconds, end_time_seconds)
                VALUES ($1, $2, $3, $4)
                "#,
            )
            .bind(new_set.region_set_id)
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

    async fn get_regions_for_region_set(
        &self,
        set_id: &RegionSetId,
    ) -> Result<Vec<DbRegion>, String> {
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
}
