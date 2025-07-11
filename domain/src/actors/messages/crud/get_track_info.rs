use serde::{Deserialize, Serialize};

use crate::track_meta::TrackMeta;

pub struct GetTrackMeta {
    pub track_id: String,
}
#[derive(Serialize,Deserialize)]
pub struct GetTrackMetaResult {
    pub track_meta: TrackMeta,
}
