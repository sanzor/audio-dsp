use serde::Serialize;

use crate::track::TrackInfo;

pub struct UpdateTrackInfo {
    pub track_info: TrackInfo,
}
#[derive(Serialize)]
pub struct UpdateTrackInfoResult {}
