use serde::{Deserialize, Serialize};

use super::source_meta::SourceMeta;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SourcePayload {
    pub canonical_audio: Vec<u8>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SourceBundle {
    pub meta: SourceMeta,
    pub payload: SourcePayload,
}
