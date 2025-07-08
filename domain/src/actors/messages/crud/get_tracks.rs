use serde::Serialize;

use crate::track::TrackInfo;

pub struct GetTracks {}

#[derive(Serialize)]
pub struct GetTracksResult {
    pub tracks: Vec<TrackInfo>,
}
