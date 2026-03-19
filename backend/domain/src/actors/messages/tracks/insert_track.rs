use serde::Serialize;

use crate::db::TrackId;
use crate::tracks::raw_track::RawTrack;

pub struct InsertTrack {
    pub track: RawTrack,
}
#[derive(Serialize)]
pub struct InsertTrackResult {
    pub track_id: TrackId,
    pub user_id: i64,
}
