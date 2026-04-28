use domain::{
    db::db_track::{DbTrack, DbTrackMeta, TrackId},
    raw_track::TrackInfo,
    update_track_info_params::UpdateTrackInfoParams,
};
use sqlx::PgPool;

use super::tracks_data_provider::TracksDataProvider;

pub struct PostgresTracksDataProvider {
    pool: PgPool,
}

impl PostgresTracksDataProvider {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl TracksDataProvider for PostgresTracksDataProvider {
    async fn get_track(&self, track_id: &TrackId) -> Result<DbTrack, String> {
        sqlx::query_as::<_, DbTrack>(
            "SELECT track_id, name, extension, length_seconds, created_at FROM tracks WHERE track_id = $1"
        )
        .bind(track_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn get_all_track_metas(&self) -> Result<Vec<DbTrackMeta>, String> {
        sqlx::query_as::<_, DbTrackMeta>(
            "SELECT track_id, name, extension, length_seconds, created_at FROM tracks ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn delete_track(&self, track_id: &TrackId) -> Result<(), String> {
        sqlx::query("DELETE FROM tracks WHERE track_id = $1")
            .bind(track_id)
            .execute(&self.pool)
            .await
            .map(|_| ())
            .map_err(|e| e.to_string())
    }

    async fn insert_track(&self, track_info: TrackInfo, project_id: i32) -> Result<DbTrack, String> {
        sqlx::query_as::<_, DbTrack>(
            "INSERT INTO tracks (name, extension, length_seconds, project_id)
             VALUES ($1, $2, $3, $4)
             RETURNING track_id, name, extension, length_seconds, created_at",
        )
        .bind(&track_info.name)
        .bind(&track_info.extension)
        .bind(track_info.length)
        .bind(project_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn copy_track(&self, source_track_id: &TrackId, new_name: &str) -> Result<DbTrack, String> {
        sqlx::query_as::<_, DbTrack>(
            "INSERT INTO tracks (name, extension, length_seconds)
             SELECT $1, extension, length_seconds FROM tracks WHERE track_id = $2
             RETURNING track_id, name, extension, length_seconds, created_at",
        )
        .bind(new_name)
        .bind(source_track_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn update_track_info(&self, track_id: &TrackId, params: UpdateTrackInfoParams) -> Result<DbTrack, String> {
        sqlx::query_as::<_, DbTrack>(
            "UPDATE tracks SET name = $2 WHERE track_id = $1
             RETURNING track_id, name, extension, length_seconds, created_at",
        )
        .bind(track_id)
        .bind(params.track_name)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }
}
