use kameo::actor::Spawn;

use crate::audio_player_actor::{
    actor::AudioPlayerActor,
    create_audio_player_actor_params::{
        CreateAudioPlayerActorParams, CreateAudioPlayerActorResult,
    },
};

pub struct PlayerFactory {}

impl PlayerFactory {
    pub fn create_audio_actor(
        &self,
        params: CreateAudioPlayerActorParams,
    ) -> Result<CreateAudioPlayerActorResult, String> {
        Ok(CreateAudioPlayerActorResult {
            audio_actor_ref: AudioPlayerActor::spawn(AudioPlayerActor::new(params)),
        })
    }
}
