use actix::{Context,Actor,Running};
use actix::prelude::*;
struct UserActor{
    
}

impl Actor for UserActor{
    type Context=Context<Self>;
    fn started(&mut self,_ctx:&mut Self::Context){

    }
    fn stopping(&mut self, ctx: &mut Self::Context) -> actix::Running {
        
    }
}