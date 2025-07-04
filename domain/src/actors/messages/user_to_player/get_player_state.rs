use crate::actors::player_state::AudioPlayerState;

pub struct GetPlayerState{ track_id:String}

pub struct GetPlayerStateResult{
    pub cursor: usize,
    pub written: usize,
    pub state: AudioPlayerState,
}