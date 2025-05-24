use super::player_states::PlayerStates;
#[derive(Clone,PartialEq)]
pub struct PlayerState {
    pub current_state: PlayerStates,
    pub cursor: usize,
}
