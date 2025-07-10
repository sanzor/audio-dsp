use std::collections::HashMap;

use crate::{
    actors::messages::player::get_player_state::GetPlayerStateResult, track_meta::TrackMeta,
};

pub struct GetUserState {}

pub struct GetUserStateResult {
    pub tracks: HashMap<String, TrackMeta>,
    pub players: HashMap<String, GetPlayerStateResult>,
}
