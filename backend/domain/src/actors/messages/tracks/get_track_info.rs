use serde::{Deserialize, Serialize};

use crate::db::TrackId;
use crate::track_meta::TrackMeta;

pub struct GetTrackMeta {
    pub track_id: TrackId,
}
#[derive(Serialize, Deserialize)]
pub struct GetTrackMetaResult {
    pub track_meta: TrackMeta,
}
