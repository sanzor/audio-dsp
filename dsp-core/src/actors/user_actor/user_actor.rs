use actix::{Context,Actor,Running};

use crate::dispatcher_enum::DispatcherEnum;

use super::user_actor_state::UserActorState;
struct UserActor{
    dispatchers:Vec<DispatcherEnum>,
    state:UserActorState
}

impl Actor for UserActor{
    type Context=Context<Self>;
    fn started(&mut self,_ctx:&mut Self::Context){
        let v=UserActor::start();
    }
    
    fn stopped(&mut self, ctx: &mut Self::Context) {
        
    }

    fn handle(&must self,msg:Message,ctx:Self::Context)->Result<
}