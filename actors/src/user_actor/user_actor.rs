use dsp_core::{command_processor::CommandProcessor, state::TracksState};
use kameo::{
    actor::ActorRef,
    prelude::{Context, Message},
    spawn, Actor,
};
use player::audio_sink::cpal_sink::CpalSink;
use std::{collections::HashMap, sync::Arc};

use crate::{
    audio_player_actor::{
        audio_player_actor::AudioPlayerActor, audio_player_actor_params::AudioPlayerActorParams,
    },
    user_actor::create_user_actor_params::CreateUserActorParams,
};
use domain::{
    actors::{
        player_command::PlayerCommand,
        player_state_query::PlayerStateQuery, 
        player_state_query_result::PlayerStateQueryResult, 
        user_crud_command::UserCrudCommand, user_crud_command_result::UserCrudCommandResult, 
        user_crud_command::UserCrudCommand, 
        user_crud_command_result::UserCrudCommandResult, 
        user_player_command::UserPlayerCommand, 
        user_player_command_result::UserPlayerCommandResult, 
        user_player_state_query::UserPlayerStateQuery, 
        user_player_state_query_result::UserPlayerStateQueryResult, 
        user_state_query::UserStateQuery, 
        user_state_query_result::UserStateQueryResult, 
        user_update_params::UserUpdateParams
    },
    dsp_message::DspMessage,
    track::TrackInfo,
    tracks_message_result::TracksMessageResult,
};

type Players = HashMap<String, ActorRef<AudioPlayerActor>>;
#[derive(Actor)]
#[actor(name = "UserActor")]
pub struct UserActor {
    id: String,
    name: String,
    email: String,
    processor: Arc<CommandProcessor>,
    track_state: TracksState,
    players: Players,
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
