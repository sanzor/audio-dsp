use domain::actors::messages::player::get_player_state::GetPlayerStateResult;

use crate::user_actor::get_player_result::GetPlayerResult;

pub struct GetAllPlayersResult {
    pub items: Vec<GetPlayerResult>,
}
