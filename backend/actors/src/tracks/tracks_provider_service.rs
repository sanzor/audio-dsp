use async_trait::async_trait;
use audiolib::utils::encode_audio_buffer_as_wav;
use domain::{
    db::db_track::DbTrack,
    raw_track::RawTrack,
    update_track_info_params::UpdateTrackInfoParams,
};
use sqlx::PgPool;
use ulid::Ulid;

use super::tracks_provider::{TrackId, TracksProvider};

pub struct PostgresTracksProvider {
    pool: PgPool,
}

impl PostgresTracksProvider {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TracksProvider for PostgresTracksProvider {
    async fn get_track(&self, track_id: &TrackId) -> Result<DbTrack, String> {
        sqlx::query_as::<_, DbTrack>(
            "SELECT track_id, name, extension, length_seconds, canonical_audio, created_at FROM tracks WHERE track_id = $1"
        )
        .bind(track_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn get_all_tracks(&self) -> Result<Vec<DbTrack>, String> {
        sqlx::query_as::<_, DbTrack>(
            "SELECT track_id, name, extension, length_seconds, canonical_audio, created_at FROM tracks"
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
        let track_id = Ulid::new().to_string();
        let canonical_audio = encode_audio_buffer_as_wav(&track.data)
            .map_err(|_| "Could not encode track as wav".to_string())?;

        sqlx::query_as::<_, DbTrack>(
            "INSERT INTO tracks (track_id, name, extension, length_seconds, canonical_audio)
             VALUES ($1, $2, $3, $4, $5)
             ON CONFLICT (track_id) DO UPDATE
             SET name = EXCLUDED.name, extension = EXCLUDED.extension,
                 length_seconds = EXCLUDED.length_seconds, canonical_audio = EXCLUDED.canonical_audio
             RETURNING track_id, name, extension, length_seconds, canonical_audio, created_at"
        )
        .bind(&track_id)
        .bind(&track.info.name)
        .bind(&track.info.extension)
        .bind(track.info.length)
        .bind(canonical_audio)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn copy_track(&self, source_track_id: &TrackId, new_name: &str) -> Result<DbTrack, String> {
        let new_track_id = Ulid::new().to_string();
        sqlx::query_as::<_, DbTrack>(
            "INSERT INTO tracks (track_id, name, extension, length_seconds, canonical_audio)
             SELECT $1, $2, extension, length_seconds, canonical_audio FROM tracks WHERE track_id = $3
             RETURNING track_id, name, extension, length_seconds, canonical_audio, created_at"
        )
        .bind(&new_track_id)
        .bind(new_name)
        .bind(source_track_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn update_track_info(&self, track_id: &TrackId, params: UpdateTrackInfoParams) -> Result<DbTrack, String> {
        sqlx::query_as::<_, DbTrack>(
            "UPDATE tracks SET name = $1 WHERE track_id = $2
             RETURNING track_id, name, extension, length_seconds, canonical_audio, created_at"
        )
        .bind(&params.track_name)
        .bind(track_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }
}
