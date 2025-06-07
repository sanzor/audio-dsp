use actix::{Actor, Context, Handler, ResponseFuture};

use crate::{
    actors::{AudioPlayerMessage, AudioPlayerResult},
    dispatcher_enum::DispatcherEnum,
};

use super::audio_player_actor_state::AudioPlayerActorState;
pub struct AudioPlayerActor {
    dispatchers: Vec<DispatcherEnum>,
    state: AudioPlayerActorState,
}

impl Actor for AudioPlayerActor {
    type Context = Context<Self>;
}

impl Handler<AudioPlayerMessage> for AudioPlayerActor {
    type Result = ResponseFuture<Result<AudioPlayerResult, String>>;

    fn handle(&mut self, msg: AudioPlayerMessage, ctx: &mut Self::Context) -> Self::Result {
        todo!()
    }
}
