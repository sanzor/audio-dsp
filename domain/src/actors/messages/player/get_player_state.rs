use crate::actors::player_state::AudioPlayerState;

pub struct GetPlayerState {}

pub struct PlayerStateResult {
    pub cursor: usize,
    pub written: usize,
    pub state: AudioPlayerState,
}
