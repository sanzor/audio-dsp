use serde::{Deserialize, Serialize};

use crate::{raw_track::TrackInfo, track_meta::TrackMeta};

pub struct GetTrackMetas {}

#[derive(Serialize, Deserialize)]
pub struct GetTracksResult {
    pub tracks: Vec<TrackMeta>,
}
