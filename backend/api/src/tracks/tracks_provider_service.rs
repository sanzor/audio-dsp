use std::sync::Arc;

use domain::{
    db::db_track::{DbTrack, DbTrackMeta, TrackId},
    raw_track::{RawTrack, TrackInfo},
    track_meta::TrackMeta,
    update_track_info_params::UpdateTrackInfoParams,
};

use super::{
    data_provider::tracks_data_provider::TracksDataProvider, tracks_provider::TracksProvider,
};

pub struct TracksProviderService {
    data: Arc<dyn TracksDataProvider>,
}

impl TracksProviderService {
    pub fn new(data: Arc<dyn TracksDataProvider>) -> Self {
        Self { data }
    }

    fn to_meta(track: &DbTrack) -> TrackMeta {
        TrackMeta {
            track_info: TrackInfo {
                name: track.name.clone(),
                extension: track.extension.clone(),
                length: track.length_seconds,
            },
            track_id: track.track_id.clone(),
        }
    }

    fn meta_from_db(track: DbTrackMeta) -> TrackMeta {
        TrackMeta {
            track_info: TrackInfo {
                name: track.name,
                extension: track.extension,
                length: track.length_seconds,
            },
            track_id: track.track_id,
        }
    }
}

#[async_trait::async_trait]
impl TracksProvider for TracksProviderService {
    async fn get_track_meta(&self, track_id: &TrackId) -> Result<TrackMeta, String> {
        let track = self.data.get_track(track_id).await?;
        Ok(Self::to_meta(&track))
    }

    async fn get_all_track_metas(&self) -> Result<Vec<TrackMeta>, String> {
        let tracks = self.data.get_all_track_metas().await?;
        Ok(tracks.into_iter().map(Self::meta_from_db).collect())
    }

    async fn insert_track(&self, track: RawTrack) -> Result<TrackMeta, String> {
        let db_track = self.data.upsert_track(track).await?;
        Ok(Self::to_meta(&db_track))
    }

    async fn delete_track(&self, track_id: &TrackId) -> Result<(), String> {
        self.data.delete_track(track_id).await
    }

    async fn copy_track(&self, track_id: &TrackId, copy_name: String) -> Result<TrackMeta, String> {
        let db_track = self.data.copy_track(track_id, &copy_name).await?;
        Ok(Self::to_meta(&db_track))
    }

    async fn update_track_info(
        &self,
        track_id: &TrackId,
        params: UpdateTrackInfoParams,
    ) -> Result<TrackMeta, String> {
        let db_track = self.data.update_track_info(track_id, params).await?;
        Ok(Self::to_meta(&db_track))
    }
}
