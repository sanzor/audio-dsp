use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum AudioPlayerState {
    Paused,
    Playing,
}
