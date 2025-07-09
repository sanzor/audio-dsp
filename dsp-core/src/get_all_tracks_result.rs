use std::collections::HashMap;

use domain::{raw_track::TrackInfo, track_meta::TrackMeta};

pub struct GetAllTrackInfosResult {
    pub track_infos: HashMap<String, TrackMeta>,
}
