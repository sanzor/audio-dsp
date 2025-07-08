use serde::Serialize;

pub struct CopyTrack {
    pub track_id: String,
    pub track_copy_name: String,
}
#[derive(Serialize)]
pub struct CopyTrackResult {
    pub copied_track_id: String,
    pub track_copy_name: String,
}
