use crate::user_actor::get_player_result::GetPlayerResult;

pub struct GetAllPlayersResult {
    pub items: Vec<GetPlayerResult>,
}
