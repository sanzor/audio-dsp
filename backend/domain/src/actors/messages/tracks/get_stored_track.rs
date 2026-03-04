use serde::{Deserialize, Serialize};

use crate::db::TrackId;
use crate::tracks::stored_track::StoredTrack;

pub struct GetStoredTrack {
    pub track_id: TrackId,
}
#[derive(Serialize, Deserialize)]
pub struct GetStoredTrackResult {
    pub track: StoredTrack,
}
