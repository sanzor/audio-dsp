use kameo::actor::ActorRef;

use crate::audio_player_actor::audio_player_actor::AudioPlayerActor;

pub struct GetPlayerResult {
    pub player_ref: ActorRef<AudioPlayerActor>,
    pub player_id: String,
}
