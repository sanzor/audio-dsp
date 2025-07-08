use crate::actors::player_state::AudioPlayerState;

pub struct UserGetPlayerState {
    pub track_id: String,
}

pub struct UserGetPlayerStateResult {
    pub cursor: usize,
    pub written: usize,
    pub state: AudioPlayerState,
}
