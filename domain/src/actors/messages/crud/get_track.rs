use crate::track::Track;

pub struct GetTrack {
    pub track_id: String,
}

pub struct GetTrackResult {
    pub track: Track,
}
