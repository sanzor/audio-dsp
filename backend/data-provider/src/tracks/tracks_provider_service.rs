use audiolib::utils::encode_audio_buffer_as_wav;
use domain::{
    db::{
        db_region::DbRegion,
        db_region_set::{DbRegionSet, RegionSetId},
        db_track::{DbTrack, TrackId},
    },
    region_set::region_set_subtree::RegionSetSubtree,
    regions::region_subtree::RegionSubtree,
    tracks::{raw_track::RawTrack, track_subtree::TrackSubtree},
    update_track_info_params::UpdateTrackInfoParams,
};
use sqlx::PgPool;
use ulid::Ulid;

use super::tracks_provider::TracksProvider;

pub struct PostgresTracksProvider {
    pool: PgPool,
}

impl PostgresTracksProvider {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl TracksProvider for PostgresTracksProvider {
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

    async fn get_all_tracks(&self) -> Result<Vec<DbTrack>, String> {
        sqlx::query_as::<_, DbTrack>(
            r#"
            SELECT track_id, name, extension, length_seconds, canonical_audio, created_at
            FROM tracks
            ORDER BY created_at DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn delete_track(&self, track_id: &TrackId) -> Result<(), String> {
        sqlx::query(r#"DELETE FROM tracks WHERE track_id = $1"#)
            .bind(track_id)
            .execute(&self.pool)
            .await
            .map(|_| ())
            .map_err(|e| e.to_string())
    }

    async fn upsert_track(&self, track: RawTrack) -> Result<DbTrack, String> {
        let track_id: TrackId = Ulid::new().to_string();
        let canonical_audio = encode_audio_buffer_as_wav(&track.data)
            .map_err(|_| "Could not encode track as wav".to_string())?;

        sqlx::query_as::<_, DbTrack>(
            r#"
            INSERT INTO tracks (track_id, name, extension, length_seconds, canonical_audio)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING track_id, name, extension, length_seconds, canonical_audio, created_at
            "#,
        )
        .bind(track_id)
        .bind(track.info.name)
        .bind(track.info.extension)
        .bind(track.info.length)
        .bind(canonical_audio)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn copy_track(&self, source_track_id: &TrackId, new_name: &str) -> Result<DbTrack, String> {
        let original = self.get_track(source_track_id).await?;
        let new_track_id: TrackId = Ulid::new().to_string();

        sqlx::query_as::<_, DbTrack>(
            r#"
            INSERT INTO tracks (track_id, name, extension, length_seconds, canonical_audio)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING track_id, name, extension, length_seconds, canonical_audio, created_at
            "#,
        )
        .bind(new_track_id)
        .bind(new_name)
        .bind(original.extension)
        .bind(original.length_seconds)
        .bind(original.canonical_audio)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn update_track_info(
        &self,
        track_id: &TrackId,
        params: UpdateTrackInfoParams,
    ) -> Result<DbTrack, String> {
        sqlx::query_as::<_, DbTrack>(
            r#"
            UPDATE tracks
            SET name = $2
            WHERE track_id = $1
            RETURNING track_id, name, extension, length_seconds, canonical_audio, created_at
            "#,
        )
        .bind(track_id)
        .bind(params.track_name)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn get_track_subtree(&self, track_id: &TrackId) -> Result<TrackSubtree, String> {
        let track = self.get_track(track_id).await?;

        let region_sets: Vec<DbRegionSet> = sqlx::query_as::<_, DbRegionSet>(
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
        .map_err(|e| e.to_string())?;

        let set_ids: Vec<RegionSetId> = region_sets.iter().map(|s| s.region_set_id.clone()).collect();

        let all_regions: Vec<DbRegion> = if set_ids.is_empty() {
            vec![]
        } else {
            sqlx::query_as::<_, DbRegion>(
                r#"
                SELECT region_id, region_set_id, name, start_time_seconds, end_time_seconds, created_at
                FROM regions
                WHERE region_set_id = ANY($1)
                ORDER BY start_time_seconds ASC
                "#,
            )
            .bind(&set_ids)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?
        };

        let assembled_sets: Vec<RegionSetSubtree> = region_sets
            .into_iter()
            .map(|set| {
                let regions: Vec<RegionSubtree> = all_regions
                    .iter()
                    .filter(|r| r.region_set_id == set.region_set_id)
                    .map(|r| RegionSubtree {
                        region_id: r.region_id.clone(),
                        region_set_id: r.region_set_id.clone(),
                        name: r.name.clone(),
                        start_time: r.start_time_seconds,
                        end_time: r.end_time_seconds,
                        graph: None,
                    })
                    .collect();
                RegionSetSubtree {
                    track_id: set.track_id,
                    track_length: set.track_length_seconds,
                    region_set_id: set.region_set_id,
                    name: set.name,
                    regions,
                }
            })
            .collect();

        Ok(TrackSubtree {
            track_id: track.track_id,
            name: track.name,
            extension: track.extension,
            length_seconds: track.length_seconds,
            region_sets: assembled_sets,
        })
    }
}
