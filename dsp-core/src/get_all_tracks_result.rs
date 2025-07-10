use std::collections::HashMap;

use domain::track_meta::TrackMeta;

pub struct GetAllTrackInfosResult {
    pub track_infos: HashMap<String, TrackMeta>,
}
