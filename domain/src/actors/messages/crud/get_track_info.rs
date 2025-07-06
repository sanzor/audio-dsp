use crate::track::TrackInfo;

pub struct GetTrackInfo {
    pub track_id: String,
}

pub struct GetTrackInfoResult {
    pub track_info: TrackInfo,
}
