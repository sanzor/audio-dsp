use std::sync::Arc;

use audiolib::utils::encode_audio_buffer_as_wav;
use domain::{
    db::db_track::{DbTrack, TrackId},
    raw_track::{RawTrack, TrackInfo},
    tracks::track_bundle::{TrackBundle, TrackPayload},
    track_meta::TrackMeta,
    update_track_info_params::UpdateTrackInfoParams,
};

use super::{
    data_provider::tracks_data_provider::TracksDataProvider,
    storage_provider::track_storage_provider::TrackStorageProvider,
    tracks_provider::TracksProvider,
};

pub struct TracksProviderService {
    data: Arc<dyn TracksDataProvider>,
    storage: Arc<dyn TrackStorageProvider>,
}

impl TracksProviderService {
    pub fn new(data: Arc<dyn TracksDataProvider>, storage: Arc<dyn TrackStorageProvider>) -> Self {
        Self { data, storage }
    }

    fn to_meta(track: &DbTrack) -> TrackMeta {
        TrackMeta {
            track_info: TrackInfo {
                name: track.name.clone(),
                extension: track.extension.clone(),
                length: track.length_seconds,
            },
            track_id: track.track_id,
        }
    }

    fn meta_from_db(track: DbTrack) -> TrackMeta {
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
    async fn get_track_meta(&self, track_id: &TrackId, workspace_id: i32) -> Result<TrackMeta, String> {
        let track = self.data.get_track(track_id, workspace_id).await?;
        Ok(Self::to_meta(&track))
    }

    async fn get_track(&self, track_id: &TrackId, workspace_id: i32) -> Result<TrackBundle, String> {
        let meta=self.data.get_track(track_id, workspace_id).await?;
        let payload=self.storage.get_track_payload(track_id).await?;
        let meta=Self::to_meta(&meta);
        Ok(TrackBundle{payload,meta})
    }

    async fn get_tracks(&self, workspace_id: i32) -> Result<Vec<TrackBundle>, String> {
        let metas = self.get_all_track_metas(workspace_id).await?;
        let mut tracks = Vec::with_capacity(metas.len());

        for meta in metas {
            let payload = self.storage.get_track_payload(&meta.track_id).await?;
            tracks.push(TrackBundle { meta, payload });
        }

        Ok(tracks)
    }

    async fn get_all_track_metas(&self, workspace_id: i32) -> Result<Vec<TrackMeta>, String> {
        let tracks = self.data.get_all_track_metas(workspace_id).await?;
        Ok(tracks.into_iter().map(Self::meta_from_db).collect())
    }

    async fn insert_track(&self, track: RawTrack, workspace_id: i32) -> Result<TrackBundle, String> {
        let canonical_audio = encode_audio_buffer_as_wav(&track.data)
            .map_err(|_| "Could not encode track as wav".to_string())?;

        let db_track = self.data.insert_track(track.info, workspace_id).await?;
        let meta = Self::to_meta(&db_track);
        let payload = TrackPayload { canonical_audio };

        if let Err(err) = self
            .storage
            .insert_track_payload(&meta.track_id, payload.clone())
            .await
        {
            let _ = self.data.delete_track(&meta.track_id, workspace_id).await;
            return Err(err);
        }

        Ok(TrackBundle { meta, payload })
    }

    async fn delete_track(&self, track_id: &TrackId, workspace_id: i32) -> Result<(), String> {
        self.data.delete_track(track_id, workspace_id).await
    }

    async fn copy_track(&self, track_id: &TrackId, copy_name: String, workspace_id: i32) -> Result<TrackMeta, String> {
        self.data.get_track(track_id, workspace_id).await?;
        let source_payload = self.storage.get_track_payload(track_id).await?;
        let db_track = self.data.copy_track(track_id, &copy_name, workspace_id).await?;
        let meta = Self::to_meta(&db_track);

        if let Err(err) = self
            .storage
            .insert_track_payload(&meta.track_id, source_payload)
            .await
        {
            let _ = self.data.delete_track(&meta.track_id, workspace_id).await;
            return Err(err);
        }

        Ok(meta)
    }

    async fn update_track_info(
        &self,
        track_id: &TrackId,
        params: UpdateTrackInfoParams,
        workspace_id: i32,
    ) -> Result<TrackMeta, String> {
        let db_track = self.data.update_track_info(track_id, params, workspace_id).await?;
        Ok(Self::to_meta(&db_track))
    }
}
