use std::collections::HashMap;

use dsp_domain::track::TrackInfo;

use crate::audio_player_actor::state_reply::AudioPlayerActorStateResult;

pub struct GetStateResult{
    pub tracks:HashMap<String,TrackInfo>,
    pub players:HashMap<String,AudioPlayerActorStateResult>
}