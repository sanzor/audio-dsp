use audiolib::utils::encode_audio_buffer_as_wav;
use domain::{
    raw_track::RawTrack, 
    stored_track::StoredTrack,
    track_meta::TrackMeta,
    update_track_info_params::UpdateTrackInfoParams
};
use ulid::Ulid;
use std::collections::HashMap;
use tokio::sync::Mutex;

use crate::{
    get_all_track_infos_result::GetAllTrackInfosResult, tracks_provider::{LocalTrackStoreProvider, TracksProvider}
};

impl LocalTrackStoreProvider {
    pub fn new() -> LocalTrackStoreProvider {
        LocalTrackStoreProvider {
            tracks: Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait::async_trait]
impl TracksProvider for LocalTrackStoreProvider {
    async fn get_track_meta(&self, track_name: &str) -> Result<TrackMeta, String> {
        let guard = self.tracks.lock().await;
        let info = guard
            .get(track_name)
            .ok_or_else(|| "err".to_string())
            .map(|track| TrackMeta {
                track_info: track.track_info.clone(),
                track_id: track.track_id.clone(),
            });

        info
    }

    async fn get_stored_track(&self,track_id:&str)->Result<StoredTrack,String>{
        let guard=self.tracks.lock().await;
        let track=match guard.get(track_id) {
            Some(tr)=>tr,
            None=> return Err("Could not find track".into())
        };
       
        Ok(StoredTrack{track_info:track.track_info.clone(),track_id:track_id.to_owned(),canonical_audio:track.canonical_audio.clone()})
    }

    async fn copy_track(&self,track_id:&str,new_name:&str)->Result<TrackMeta,String>{
    
        let mut guard=self.tracks.lock().await;
        let original=match guard.get(track_id){
            Some(tr)=>tr,
            None=>return Err("Could not find track".into())
        };

        let new_track_id=Ulid::new().to_string();
        let mut new_track_info=original.track_info.clone();
        new_track_info.name=new_name.to_owned();


        let copy=StoredTrack{
            track_id:new_track_id.to_string(),
            track_info:new_track_info.clone(),
            canonical_audio:original.canonical_audio.clone()
        };
       
        match guard.insert(new_track_id.to_string(),copy){
            Some(_)=>Err("Could not insert new track".into()),
            None=>Ok(TrackMeta { track_info: new_track_info, track_id: new_track_id.to_string() })
        }
        
    }
    async fn update_track_info(
        &self,
        track_id: &str,
        updated_info: UpdateTrackInfoParams,
    ) -> Result<TrackMeta, String> {
        let mut remove_guard = self.tracks.lock().await;
        let track_to_update = match remove_guard.get_mut(track_id) {
            None => return Err("Could not find track".into()),
            Some(i) => i,
        };
       
        track_to_update.track_info.name =updated_info.track_name;
       
        Ok(TrackMeta {
            track_info: track_to_update.track_info.clone(),
            track_id: track_to_update.track_id.clone(),
        })
    }


    async fn get_all_track_infos(&self) -> Result<GetAllTrackInfosResult, String> {
        let guard = self.tracks.lock().await;
        let mut hash_map = HashMap::new();
        for (key, track) in guard.iter() {
            hash_map.insert(
                key.to_string(),
                TrackMeta {
                    track_id: key.clone(),
                    track_info: track.track_info.clone(),
                },
            );
        }
        Ok(GetAllTrackInfosResult {
            track_infos: hash_map,
        })
    }

    async fn delete_track(&self, track_name: &str) -> Result<(), String> {
        let mut guard = self.tracks.lock().await;
        guard
            .remove(track_name)
            .ok_or_else(|| "could not find key".into())
            .map(|v| ())
    }
    async fn upsert_track(&self, tr: RawTrack) -> Result<TrackMeta, String> {
        let id = Ulid::new().to_string();
        let canonical_audio=match encode_audio_buffer_as_wav(&tr.data){
            Ok(c)=>c,
            Err(e)=>return Err("Could not store track".into())
        };
        let track = StoredTrack { track_id: id.clone(), track_info:tr.info.clone(), canonical_audio: canonical_audio };
        let mut guard = self.tracks.lock().await;
        match guard.insert(id.clone(), track) {
            None => Ok(TrackMeta {
                track_info: tr.info.clone(),
                track_id: id,
            }),
            Some(_) => Err("Key already exists".into()),
        }
    }
}
