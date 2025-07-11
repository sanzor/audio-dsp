use serde::Serialize;

use crate::{raw_track::TrackInfo, track_meta::TrackMeta};

pub struct UpdateTrackInfo {
    pub track_id:String,
    pub track_info: TrackInfo,
}
#[derive(Serialize)]
pub struct UpdateTrackInfoResult {
    pub track_meta: TrackMeta,
}
