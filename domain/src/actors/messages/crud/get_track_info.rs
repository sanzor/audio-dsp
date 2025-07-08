use serde::Serialize;

use crate::track::TrackInfo;

pub struct GetTrackInfo {
    pub track_id: String,
}
#[derive(Serialize)]
pub struct GetTrackInfoResult {
    pub track_info: TrackInfo,
}
