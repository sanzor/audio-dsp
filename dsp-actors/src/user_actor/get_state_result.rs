use std::collections::HashMap;

use dsp_domain::track::TrackInfo;

use crate::audio_player_actor::state_reply::AudioPlayerActorStateResult;

pub struct GetStateResult{
    tracks:HashMap<String,TrackInfo>,
    players:HashMap<String,AudioPlayerActorStateResult>
}