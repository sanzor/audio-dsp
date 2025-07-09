use std::collections::HashMap;

use domain::{
    raw_track::{RawTrack, TrackInfo, TrackRef, TrackRefMut},
    track::Track,
    track_meta::TrackMeta,
};

use crate::{
    get_all_tracks_result::GetAllTrackInfosResult,
    tracks_provider::{LocalTrackStoreProvider, TracksProvider},
};

impl LocalTrackStoreProvider {
    pub fn new() -> LocalTrackStoreProvider {
        LocalTrackStoreProvider {
            tracks: HashMap::new(),
        }
    }
}

#[async_trait::async_trait]
impl TracksProvider for LocalTrackStoreProvider {
    async fn get_track_meta(&self, track_name: &str) -> Result<TrackMeta, String> {
        let info = self
            .tracks
            .get(track_name)
            .ok_or_else(|| "err".to_string())
            .map(|track| TrackMeta {
                track_info: track.track_info.clone(),
                track_id: track.track_id.clone(),
            });

        info
    }
    async fn update_track_info(&mut self, track_info: TrackInfo) -> Result<TrackMeta, String> {
        let mut track = match self.tracks.remove(&track_info.name) {
            None => return Err("Could not find track".into()),
            Some(i) => i,
        };
        track.track_info = track_info;
        Ok(TrackMeta {
            track_info: track.track_info.clone(),
            track_id: track.track_id.clone(),
        })
    }
    async fn get_track_ref(&self, track_name: &str) -> Result<TrackRef, String> {
        self.tracks
            .get(track_name)
            .ok_or_else(|| "".into())
            .map(|track| TrackRef { inner: track })
    }

    async fn get_track_ref_mut(&mut self, track_name: &str) -> Result<TrackRefMut, String> {
        self.tracks
            .get_mut(track_name)
            .ok_or_else(|| "".into())
            .map(|track| TrackRefMut { inner: track })
    }

    async fn get_track_copy(&self, track_name: &str) -> Result<RawTrack, String> {
        self.tracks
            .get(track_name)
            .ok_or_else(|| "".into())
            .map(|track| RawTrack {
                info: track.track_info.clone(),
                data: track.data.clone(),
            })
    }
    async fn get_all_track_infos(&self) -> Result<GetAllTrackInfosResult, String> {
        let mut hash_map = HashMap::new();
        for (key, track) in self.tracks.iter() {
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

    async fn delete_track(&mut self, track_name: &str) -> Result<(), String> {
        self.tracks
            .remove(track_name)
            .ok_or_else(|| "could not find key".into())
            .map(|v| ())
    }
    async fn upsert_track(&mut self, tr: RawTrack) -> Result<TrackMeta, String> {
        let id = tr.info.name.clone();
        let track = Track {
            data: tr.data,
            track_id: id.clone(),
            track_info: tr.info.clone(),
        };
        match self.tracks.insert(id.clone(), track) {
            None => Ok(TrackMeta {
                track_info: tr.info.clone(),
                track_id: id,
            }),
            Some(_) => Err("Key already exists".into()),
        }
    }
}
