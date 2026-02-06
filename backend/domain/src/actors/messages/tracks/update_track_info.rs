use serde::Serialize;

use crate::track_meta::TrackMeta;

pub struct UpdateTrackInfo {
    pub track_id: String,
    pub name: String,
}
#[derive(Serialize)]
pub struct UpdateTrackInfoResult {
    pub track_meta: TrackMeta,
}
