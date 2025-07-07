use crate::track::TrackInfo;

pub struct GetTracks {}

pub struct GetTracksResult {
    pub tracks: Vec<TrackInfo>,
}
