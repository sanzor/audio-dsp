use crate::raw_track::TrackInfo;

// Option A: In-memory only
pub struct StoredTrack {
    pub track_id: String,
    pub track_info: TrackInfo,
    pub canonical_audio: Vec<u8>, // Always in memory
}