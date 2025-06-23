use crate::actors::player_state::AudioPlayerState;

pub struct UserPlayerStateQueryResult {
    pub cursor: usize,
    pub written: usize,
    pub state: AudioPlayerState,
}
