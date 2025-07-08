use serde::Serialize;

use crate::track::Track;

pub struct InsertTrack {
    pub track: Track,
}
#[derive(Serialize)]
pub struct InsertTrackResult {}
