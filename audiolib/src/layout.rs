use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Channels {
    Mono = 1,
    Stereo = 2,
}
