use data_provider::{
    get_all_track_infos_result::GetAllTrackInfosResult, tracks_provider::TracksProvider,
};
use domain::{
    raw_track::RawTrack, stored_track::StoredTrack, track_meta::TrackMeta,
    update_track_info_params::UpdateTrackInfoParams,
};
use dtos::db::track_subtree::TrackSubtree;

pub struct TrackDbProvider;

#[async_trait::async_trait]
impl TracksProvider for TrackDbProvider {
    async fn get_track_meta(&self, track_name: &str) -> Result<TrackMeta, String> {
        let _ = (self, track_name);
        todo!()
    }

    async fn get_stored_track(&self, track_id: &str) -> Result<StoredTrack, String> {
        let _ = (self, track_id);
        todo!()
    }

    async fn get_all_track_infos(&self) -> Result<GetAllTrackInfosResult, String> {
        let _ = self;
        todo!()
    }

    async fn delete_track(&self, track_name: &str) -> Result<(), String> {
        let _ = (self, track_name);
        todo!()
    }

    async fn upsert_track(&self, track: RawTrack) -> Result<TrackMeta, String> {
        let _ = (self, track);
        todo!()
    }

    async fn copy_track(
        &self,
        source_track_id: &str,
        new_name: &str,
    ) -> Result<TrackSubtree, String> {
        let _ = (self, source_track_id, new_name);
        todo!()
    }

    async fn fetch_subtree(&self, track_id: &str) -> Result<TrackSubtree, String> {
        let _ = (self, track_id);
        todo!()
    }

    async fn update_track_info(
        &self,
        track_id: &str,
        updated_track_info: UpdateTrackInfoParams,
    ) -> Result<TrackMeta, String> {
        let _ = (self, track_id, updated_track_info);
        todo!()
    }
}
