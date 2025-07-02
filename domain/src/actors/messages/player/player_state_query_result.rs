use serde::{Deserialize, Serialize};

use crate::actors::player_state::AudioPlayerState;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlayerStateQueryResult {
    pub cursor: usize,
    pub written: usize,
    pub state: AudioPlayerState,
}
