use dsp_core::{command_processor::CommandProcessor, state::TracksState};
use kameo::{
    actor::ActorRef,
    prelude::{Context, Message},
    Actor,
};
use std::{collections::HashMap, sync::Arc};

use crate::{
    audio_player_actor::
        audio_player_actor::AudioPlayerActor
    ,
    user_actor::create_user_actor_params::CreateUserActorParams,
};
use domain::{
    dsp_message::DspMessage,
    tracks_message_result::TracksMessageResult,
};

type Players = HashMap<String, ActorRef<AudioPlayerActor>>;
#[derive(Actor)]
#[actor(name = "UserActor")]
pub struct UserActor {
    pub(crate)id: String,
    pub(crate)name: String,
    pub(crate)email: String,
    pub(crate)processor: Arc<CommandProcessor>,
    pub(crate)track_state: TracksState,
    pub(crate)players: Players,
}

impl UserActor {
    pub fn new(actor_params: CreateUserActorParams) -> UserActor {
        UserActor {
            id: actor_params.user_data.id,
            email: actor_params.user_data.email,
            name: actor_params.user_data.name,
            processor: actor_params.processor,
            track_state: actor_params.tracks,
            players: actor_params.players,
        }
    }
}

impl Message<DspMessage> for UserActor {
    type Reply = Result<TracksMessageResult, String>;

    // async move variant
    async fn handle(
        &mut self,
        msg: DspMessage,
        ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        let v = self
            .processor
            .process_crud_command(msg, &mut self.track_state)
            .await;
        v
    }
}
