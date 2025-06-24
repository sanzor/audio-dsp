use crate::actors::player_state::AudioPlayerState;
#[derive(Debug)]
pub struct UserPlayerStateQueryResult {
    pub cursor: usize,
    pub written: usize,
    pub state: AudioPlayerState,
}
