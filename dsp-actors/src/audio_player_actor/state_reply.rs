use serde::{Deserialize, Serialize};

use crate::audio_player_actor::audio_player_actor::AudioPlayerState;
#[derive(Serialize,Deserialize)]
pub struct AudioPlayerActorStateResult{
    pub cursor:usize,
    pub written:usize,
    pub state:AudioPlayerState
}