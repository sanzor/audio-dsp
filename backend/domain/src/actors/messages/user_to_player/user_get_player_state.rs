use crate::actors::player_state::AudioPlayerState;
use crate::db::TrackId;

pub struct UserGetPlayerState {
    pub track_id: TrackId,
}

pub struct UserGetPlayerStateResult {
    pub cursor: usize,
    pub written: usize,
    pub state: AudioPlayerState,
    pub sinks: Vec<String>,
}
