use domain::{db::db_track::TrackId, raw_track::TrackInfo, tracks::stored_track::StoredTrack};
use sqlx::PgPool;

use super::stored_tracks_data_provider::StoredTracksDataProvider;

pub struct PostgresStoredTracksDataProvider {
    pool: PgPool,
}

impl PostgresStoredTracksDataProvider {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl StoredTracksDataProvider for PostgresStoredTracksDataProvider {
    async fn get_stored_track(&self, track_id: &TrackId) -> Result<StoredTrack, String> {
        #[derive(sqlx::FromRow)]
        struct Row {
            track_id: TrackId,
            name: String,
            extension: String,
            length_seconds: f32,
            data: Vec<u8>,
        }

        let row = sqlx::query_as::<_, Row>(
            "SELECT t.track_id, t.name, t.extension, t.length_seconds, ts.data
             FROM tracks t
             JOIN track_storage ts ON ts.track_id = t.track_id
             WHERE t.track_id = $1",
        )
        .bind(track_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(StoredTrack {
            track_id: row.track_id,
            track_info: TrackInfo {
                name: row.name,
                extension: row.extension,
                length: row.length_seconds,
            },
            canonical_audio: row.data,
        })
    }

    async fn insert_track_to_storage(
        &self,
        track_id: &TrackId,
        canonical_audio: Vec<u8>,
    ) -> Result<(), String> {
        sqlx::query("INSERT INTO track_storage (track_id, data) VALUES ($1, $2)")
            .bind(track_id)
            .bind(canonical_audio)
            .execute(&self.pool)
            .await
            .map(|_| ())
            .map_err(|e| e.to_string())
    }
}
