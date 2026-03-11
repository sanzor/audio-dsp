use std::collections::HashMap;

use audiolib::utils::encode_audio_buffer_as_wav;
use actors::{
    region_sets::region_sets_provider::RegionSetsProvider,
    tracks::tracks_provider::TracksProvider,
};
use domain::{
    db::{
        db_region::{DbRegion, RegionId},
        db_region_set::{DbRegionSet, RegionSetId},
        db_track::{DbTrack, TrackId},
    },
    raw_track::RawTrack,
    region_set::{
        copy_region_set_params::CopyRegionSetParams,
        create_region_set_params::CreateRegionSetParams,
        edit_region_set_params::EditRegionSetParams,
    },
    regions::{
        add_region_params::AddRegionParams, copy_region_params::CopyRegionParams,
        delete_region_params::DeleteRegionParams, edit_region_params::EditRegionParams,
    },
    update_track_info_params::UpdateTrackInfoParams,
};
use tokio::sync::Mutex;
use ulid::Ulid;

pub struct StubTracksProvider {
    tracks: Mutex<HashMap<TrackId, DbTrack>>,
}

impl StubTracksProvider {
    pub fn new() -> Self {
        Self {
            tracks: Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait::async_trait]
impl TracksProvider for StubTracksProvider {
    async fn get_track(&self, track_id: &TrackId) -> Result<DbTrack, String> {
        let guard = self.tracks.lock().await;
        guard
            .get(track_id)
            .cloned()
            .ok_or_else(|| "Could not find track".to_string())
    }

    async fn get_all_tracks(&self) -> Result<Vec<DbTrack>, String> {
        let guard = self.tracks.lock().await;
        Ok(guard.values().cloned().collect())
    }

    async fn delete_track(&self, track_id: &TrackId) -> Result<(), String> {
        let mut guard = self.tracks.lock().await;
        guard.remove(track_id);
        Ok(())
    }

    async fn upsert_track(&self, track: RawTrack) -> Result<DbTrack, String> {
        let track_id = Ulid::new().to_string();
        let canonical_audio = encode_audio_buffer_as_wav(&track.data)
            .map_err(|_| "Could not encode track as wav".to_string())?;

        let db_track = DbTrack {
            track_id: track_id.clone(),
            name: track.info.name,
            extension: track.info.extension,
            length_seconds: track.info.length,
            canonical_audio,
            created_at: chrono::Utc::now(),
        };

        let mut guard = self.tracks.lock().await;
        guard.insert(track_id, db_track.clone());
        Ok(db_track)
    }

    async fn copy_track(&self, source_track_id: &TrackId, new_name: &str) -> Result<DbTrack, String> {
        let original = self.get_track(source_track_id).await?;
        let new_track_id = Ulid::new().to_string();

        let copy = DbTrack {
            track_id: new_track_id.clone(),
            name: new_name.to_string(),
            extension: original.extension,
            length_seconds: original.length_seconds,
            canonical_audio: original.canonical_audio,
            created_at: chrono::Utc::now(),
        };

        let mut guard = self.tracks.lock().await;
        guard.insert(new_track_id, copy.clone());
        Ok(copy)
    }

    async fn update_track_info(
        &self,
        track_id: &TrackId,
        updated_track_info: UpdateTrackInfoParams,
    ) -> Result<DbTrack, String> {
        let mut guard = self.tracks.lock().await;
        let track = guard
            .get_mut(track_id)
            .ok_or_else(|| "Could not find track".to_string())?;
        track.name = updated_track_info.track_name;
        Ok(track.clone())
    }
}

pub struct StubRegionSetsProvider {
    sets: Mutex<HashMap<RegionSetId, DbRegionSet>>,
    regions: Mutex<HashMap<RegionSetId, Vec<DbRegion>>>,
}

impl StubRegionSetsProvider {
    pub fn new() -> Self {
        Self {
            sets: Mutex::new(HashMap::new()),
            regions: Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait::async_trait]
impl RegionSetsProvider for StubRegionSetsProvider {
    async fn create_region_set(&self, params: CreateRegionSetParams) -> Result<DbRegionSet, String> {
        let id = Ulid::new().to_string();
        let set = DbRegionSet {
            region_set_id: id.clone(),
            track_id: params.track_id,
            name: params.name.unwrap_or_else(|| "set".into()),
            track_length_seconds: params.track_length,
            created_at: chrono::Utc::now(),
        };
        let mut guard = self.sets.lock().await;
        guard.insert(id.clone(), set.clone());
        Ok(set)
    }

    async fn get_region_set(&self, set_id: &RegionSetId) -> Result<DbRegionSet, String> {
        let guard = self.sets.lock().await;
        guard
            .get(set_id)
            .cloned()
            .ok_or_else(|| "Could not find region set".to_string())
    }

    async fn get_region_sets_for_track(&self, track_id: &TrackId) -> Result<Vec<DbRegionSet>, String> {
        let guard = self.sets.lock().await;
        Ok(guard
            .values()
            .filter(|s| &s.track_id == track_id)
            .cloned()
            .collect())
    }

    async fn get_region_sets(&self) -> Result<Vec<DbRegionSet>, String> {
        let guard = self.sets.lock().await;
        Ok(guard.values().cloned().collect())
    }

    async fn edit_region_set(&self, params: EditRegionSetParams) -> Result<DbRegionSet, String> {
        let mut guard = self.sets.lock().await;
        let set = guard
            .get_mut(&params.region_set_id)
            .ok_or_else(|| "Could not find region set".to_string())?;
        if let Some(name) = params.name {
            set.name = name;
        }
        Ok(set.clone())
    }

    async fn delete_region_set(&self, set_id: &RegionSetId) -> Result<(), String> {
        let mut guard = self.sets.lock().await;
        guard.remove(set_id);
        Ok(())
    }

    async fn copy_region_set(&self, params: CopyRegionSetParams) -> Result<DbRegionSet, String> {
        let source = self.get_region_set(&params.region_set_id).await?;
        let id = Ulid::new().to_string();
        let set = DbRegionSet {
            region_set_id: id.clone(),
            track_id: source.track_id,
            name: params.region_set_name,
            track_length_seconds: source.track_length_seconds,
            created_at: chrono::Utc::now(),
        };
        let mut guard = self.sets.lock().await;
        guard.insert(id, set.clone());
        Ok(set)
    }

    async fn get_regions_for_region_set(&self, set_id: &RegionSetId) -> Result<Vec<DbRegion>, String> {
        let guard = self.regions.lock().await;
        Ok(guard.get(set_id).cloned().unwrap_or_default())
    }

    async fn add_region(&self, params: AddRegionParams) -> Result<DbRegion, String> {
        let region_id = Ulid::new().to_string();
        let region = DbRegion {
            region_id,
            region_set_id: params.region_set_id.clone(),
            name: params.name,
            start_time_seconds: params.start_time,
            end_time_seconds: params.end_time,
            created_at: chrono::Utc::now(),
        };
        let mut guard = self.regions.lock().await;
        guard
            .entry(params.region_set_id)
            .or_default()
            .push(region.clone());
        Ok(region)
    }

    async fn edit_region(&self, params: EditRegionParams) -> Result<DbRegion, String> {
        let mut guard = self.regions.lock().await;
        let regions = guard
            .get_mut(&params.region_set_id)
            .ok_or_else(|| "Could not find region set".to_string())?;
        let region = regions
            .iter_mut()
            .find(|r| r.region_id == params.region_id)
            .ok_or_else(|| "Could not find region".to_string())?;
        if let Some(name) = params.name {
            region.name = name;
        }
        if let Some(start) = params.start_time {
            region.start_time_seconds = start;
        }
        if let Some(end) = params.end_time {
            region.end_time_seconds = end;
        }
        Ok(region.clone())
    }

    async fn delete_region(&self, params: DeleteRegionParams) -> Result<(), String> {
        let mut guard = self.regions.lock().await;
        if let Some(regions) = guard.get_mut(&params.region_set_id) {
            regions.retain(|r| r.region_id != params.region_id);
        }
        Ok(())
    }

    async fn copy_region(&self, params: CopyRegionParams) -> Result<DbRegion, String> {
        let source = self.get_region(&params.source_region_id).await?;
        let region_id = Ulid::new().to_string();
        let region = DbRegion {
            region_id,
            region_set_id: params.destination_region_set_id.clone(),
            name: params.region_copy_name,
            start_time_seconds: source.start_time_seconds,
            end_time_seconds: source.end_time_seconds,
            created_at: chrono::Utc::now(),
        };
        let mut guard = self.regions.lock().await;
        guard
            .entry(params.destination_region_set_id)
            .or_default()
            .push(region.clone());
        Ok(region)
    }

    async fn get_region(&self, region_id: &RegionId) -> Result<DbRegion, String> {
        let guard = self.regions.lock().await;
        for regions in guard.values() {
            if let Some(r) = regions.iter().find(|r| &r.region_id == region_id) {
                return Ok(r.clone());
            }
        }
        Err("Could not find region".to_string())
    }
}

