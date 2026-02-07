use serde::{Deserialize, Serialize};

use crate::{db::TrackId, tracks::track_info::TrackInfo};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TrackMeta {
    pub track_info: TrackInfo,
    pub track_id: TrackId,
}
