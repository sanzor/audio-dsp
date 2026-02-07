use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TrackInfo {
    pub name: String,
    pub extension: String,
    pub length: f32,
}
