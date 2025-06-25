use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum AudioPlayerState {
    Paused,
    Playing,
}
