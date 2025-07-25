use domain::{
    raw_track::{RawTrack, TrackInfo},
    track::Track,
    track_meta::TrackMeta,
};
use tokio::sync::Mutex;
use std::collections::HashMap;
use ulid::Ulid;

use crate::{
    get_all_tracks_result::GetAllTrackInfosResult,
    tracks_provider::{LocalTrackStoreProvider, TracksProvider},
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
        let guard=self.tracks.lock().await;
        let info = guard
            .get(track_name)
            .ok_or_else(|| "err".to_string())
            .map(|track| TrackMeta {
                track_info: track.track_info.clone(),
                track_id: track.track_id.clone(),
            });

        info
    }
    async fn update_track_info(
        & self,
        track_id: &str,
        track_info: TrackInfo,
    ) -> Result<TrackMeta, String> {
        let mut guard = self.tracks.lock().await;
        let mut track=match guard.remove(track_id) {
            None => return Err("Could not find track".into()),
            Some(i) => i,
        };
        track.track_info = track_info;
        Ok(TrackMeta {
            track_info: track.track_info.clone(),
            track_id: track.track_id.clone(),
        })
    }


    async fn get_track_copy(&self, track_name: &str) -> Result<RawTrack, String> {
        let guard=self.tracks.lock().await;
        guard
            .get(track_name)
            .ok_or_else(|| "".into())
            .map(|track| RawTrack {
                info: track.track_info.clone(),
                data: track.data.clone(),
            })
    }
    async fn get_all_track_infos(&self) -> Result<GetAllTrackInfosResult, String> {
        let guard=self.tracks.lock().await;
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
        let mut guard=self.tracks.lock().await;
        guard
            .remove(track_name)
            .ok_or_else(|| "could not find key".into())
            .map(|v| ())
    }
    async fn upsert_track(&self, tr: RawTrack) -> Result<TrackMeta, String> {
        let id = Ulid::new().to_string();
        let track = Track {
            data: tr.data,
            track_id: id.clone(),
            track_info: tr.info.clone(),
        };
        let mut guard=self.tracks.lock().await;
        match guard.insert(id.clone(), track) {
            None => Ok(TrackMeta {
                track_info: tr.info.clone(),
                track_id: id,
            }),
            Some(_) => Err("Key already exists".into()),
        }
    }
}
