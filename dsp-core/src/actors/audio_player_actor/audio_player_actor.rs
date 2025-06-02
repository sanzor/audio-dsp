use actix::{Actor, Context, Handler};
use dsp_domain::dsp_message::DspMessage;

use crate::dispatcher_enum::DispatcherEnum;

use super::audio_player_actor_state::AudioPlayerActorState;
struct AudioPlayerActor{
    dispatchers:Vec<DispatcherEnum>,
    state:AudioPlayerActorState
}

impl Actor for AudioPlayerActor{
    type Context=Context<Self>;
    fn stopped(&mut self, ctx: &mut Self::Context) {
        
    }

}

impl Handler<DspMessage> for AudioPlayerActor{
    type Result=ResponseFuture

    fn handle(&mut self, msg: DspMessage, ctx: &mut Self::Context) -> Self::Result {
        todo!()
    }
}

fn start(){
    let v=AudioPlayerActor::create(|ctx| AudioPlayerActor{state:AudioPlayerActorState::new(),dispatchers:vec![]});
    v.send(msg)

}