use audiolib::utils::encode_audio_buffer_as_wav;
use domain::{
    db::db_track::{DbTrack, DbTrackMeta, TrackId},
    raw_track::RawTrack,
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

    async fn upsert_track(&self, track: RawTrack) -> Result<DbTrack, String> {
        let canonical_audio = encode_audio_buffer_as_wav(&track.data)
            .map_err(|_| "Could not encode track as wav".to_string())?;

        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        let db_track = sqlx::query_as::<_, DbTrack>(
            "INSERT INTO tracks (name, extension, length_seconds)
             VALUES ($1, $2, $3)
             RETURNING track_id, name, extension, length_seconds, created_at",
        )
        .bind(&track.info.name)
        .bind(&track.info.extension)
        .bind(track.info.length)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        sqlx::query(
            "INSERT INTO track_storage (track_id, data) VALUES ($1, $2)",
        )
        .bind(db_track.track_id)
        .bind(&canonical_audio)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;

        Ok(db_track)
    }

    async fn copy_track(&self, source_track_id: &TrackId, new_name: &str) -> Result<DbTrack, String> {
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        let db_track = sqlx::query_as::<_, DbTrack>(
            "INSERT INTO tracks (name, extension, length_seconds)
             SELECT $1, extension, length_seconds FROM tracks WHERE track_id = $2
             RETURNING track_id, name, extension, length_seconds, created_at",
        )
        .bind(new_name)
        .bind(source_track_id)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        sqlx::query(
            "INSERT INTO track_storage (track_id, data)
             SELECT $1, data FROM track_storage WHERE track_id = $2",
        )
        .bind(db_track.track_id)
        .bind(source_track_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;

        Ok(db_track)
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
