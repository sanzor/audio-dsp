use std::collections::HashMap;

use serde::Serialize;

use crate::{actors::player_state::AudioPlayerState, track::TrackInfo};

pub struct GetUserState{

}

pub struct GetUserStateResult {
    pub tracks: HashMap<String, TrackInfo>,
    pub players: HashMap<String, PlayerStateQueryResult>,
}

#[derive(Serialize, Clone, Debug)]
pub struct PlayerStateQueryResult {
    pub cursor: usize,
    pub written: usize,
    pub state: AudioPlayerState,
}

