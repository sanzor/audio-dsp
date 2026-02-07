use serde::Serialize;

use crate::db::TrackId;

pub struct DeleteTrack {
    pub track_id: TrackId,
}
#[derive(Serialize)]
pub struct DeleteTrackResult {}
