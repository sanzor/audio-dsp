use actix::{Actor, Context, Handler, ResponseFuture};
use dsp_domain::{dsp_message::DspMessage, dsp_message_result::DspMessageResult};

use crate::{command_processor::CommandProcessor, dispatcher_enum::DispatcherEnum};

use super::audio_player_actor_state::AudioPlayerActorState;
pub struct AudioPlayerActor{
    dispatchers:Vec<DispatcherEnum>,
    state:AudioPlayerActorState
}

impl Actor for AudioPlayerActor{
    type Context=Context<Self>;
}

impl Handler<DspMessage> for AudioPlayerActor{
    type Result=ResponseFuture<DspMessageResult>;

    fn handle(&mut self, msg: DspMessage, ctx: &mut Self::Context) -> Self::Result {
        todo!()
    }
}

fn start(){
    let dispatcher=CommandProcessor::create_processor();
    let v=AudioPlayerActor::create(|ctx| AudioPlayerActor{state:AudioPlayerActorState::new(),dispatchers:vec![]});


}