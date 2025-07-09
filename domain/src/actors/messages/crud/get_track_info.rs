use serde::Serialize;

use crate::{raw_track::TrackInfo, track_meta::TrackMeta};

pub struct GetTrackMeta {
    pub track_id: String,
}
#[derive(Serialize)]
pub struct GetTrackMetaResult {
    pub track_meta: TrackMeta,
}
