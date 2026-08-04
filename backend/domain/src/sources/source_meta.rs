use serde::{Deserialize, Serialize};

use crate::{db::SourceId, sources::source_info::SourceInfo};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SourceMeta {
    pub source_info: SourceInfo,
    pub source_id: SourceId,
}
