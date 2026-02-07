use serde::Serialize;

use crate::db::TrackId;

pub struct CopyTrack {
    pub track_id: TrackId,
    pub track_copy_name: String,
}
#[derive(Serialize)]
pub struct CopyTrackResult {
    pub copied_track_id: TrackId,
    pub track_copy_name: String,
}
