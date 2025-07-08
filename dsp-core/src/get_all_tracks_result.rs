use std::collections::HashMap;

use domain::track::TrackInfo;

pub struct GetAllTrackInfosResult {
    pub track_infos: HashMap<String, TrackInfo>,
}
