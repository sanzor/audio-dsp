use serde::Serialize;

pub struct DeleteTrack {
    pub track_id: String,
}
#[derive(Serialize)]
pub struct DeleteTrackResult {}
