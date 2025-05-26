use super::player_states::PlayerStates;
#[derive(Clone, PartialEq)]
pub struct PlayerState {
    pub(crate) current_state: PlayerStates,
    pub cursor: usize,
    pub frames_written: usize,
}
