use actix::{Actor, AsyncContext, Context};

use crate::dispatcher_enum::DispatcherEnum;

use super::audio_player_actor_state::AudioPlayerActorState;
struct AudioPlayerActor{
    dispatchers:Vec<DispatcherEnum>,
    state:AudioPlayerActorState
}

impl Actor for AudioPlayerActor{
    type Context=Context<Self>;
    fn started(&mut self,_ctx:&mut Self::Context){
        let v=self.state
    }
    
    fn stopped(&mut self, ctx: &mut Self::Context) {
        
    }
    fn handle(&mut self,msg:
    
    )

    
}

fn start(){
    let v=AudioPlayerActor::create(|ctx| AudioPlayerActor{state:AudioPlayerActorState::new(),dispatchers:vec![]});
    v.send(msg)

}