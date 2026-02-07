use serde::Serialize;

use crate::db::TrackId;
use crate::track_meta::TrackMeta;

pub struct UpdateTrackInfo {
    pub track_id: TrackId,
    pub name: String,
}
#[derive(Serialize)]
pub struct UpdateTrackInfoResult {
    pub track_meta: TrackMeta,
}
