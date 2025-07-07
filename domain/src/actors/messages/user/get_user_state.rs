use std::collections::HashMap;

use crate::{actors::messages::player::get_player_state::PlayerStateResult, track::TrackInfo};

pub struct GetUserState {}

pub struct GetUserStateResult {
    pub tracks: HashMap<String, TrackInfo>,
    pub players: HashMap<String, PlayerStateResult>,
}
