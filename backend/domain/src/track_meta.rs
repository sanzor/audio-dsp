use serde::{Deserialize, Serialize};

use crate::raw_track::TrackInfo;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TrackMeta {
    pub track_info: TrackInfo,
    pub track_id: String,
}
