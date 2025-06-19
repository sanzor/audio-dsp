use crate::audio_player_actor::audio_player_actor::AudioPlayerState;

pub struct StateReply{
    pub cursor:usize,
    pub state:AudioPlayerState
}