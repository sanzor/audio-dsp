use serde::{Deserialize, Serialize};

use crate::actors::player_state::AudioPlayerState;

pub struct GetPlayerState {}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GetPlayerStateResult {
    pub cursor: usize,
    pub written: usize,
    pub state: AudioPlayerState,
    pub sinks: Vec<String>,
}
