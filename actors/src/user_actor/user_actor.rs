use dsp_core::{
    command_processor::CommandProcessor,
    state::{TracksProvider, TracksState},
};
use kameo::{
    actor::ActorRef,
    prelude::{Context, Message},
    Actor,
};
use std::{collections::HashMap, sync::Arc};

use crate::{
    audio_player_actor::audio_player_actor::{AudioPlayerActor},
    user_actor::{
        create_user_actor_params::CreateUserActorParams, crud::TrackOperations,
        players_provider::PlayersProvider,
    },
};
use domain::{dsp_message::DspMessage, tracks_message_result::TracksMessageResult};

type Players = HashMap<String, ActorRef<AudioPlayerActor>>;
#[derive(Actor)]
#[actor(name = "UserActor")]
pub struct UserActor {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) email: String,
    pub(crate) processor: Arc<CommandProcessor>,
    pub(crate) tracks_provider: Box<dyn TracksProvider + Send + Sync>,
    pub(crate) players_provider: Box<dyn PlayersProvider + Send + Sync>,
}

impl UserActor {
    pub fn new(actor_params: CreateUserActorParams) -> UserActor {
        UserActor {
            id: actor_params.user_data.id,
            email: actor_params.user_data.email,
            name: actor_params.user_data.name,
            processor: actor_params.processor,
            tracks_provider: &actor_params.tracks_provider,
            players_provider: actor_params.players,
        }
    }
}
