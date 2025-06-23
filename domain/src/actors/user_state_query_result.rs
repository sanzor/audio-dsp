use std::collections::HashMap;

use crate::{actors::player_state_query_result::PlayerStateQueryResult, track::TrackInfo};

pub struct UserStateQueryResult {
    pub tracks: HashMap<String, TrackInfo>,
    pub players: HashMap<String, PlayerStateQueryResult>,
}
