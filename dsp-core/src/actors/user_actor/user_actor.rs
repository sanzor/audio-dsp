use actix::{Actor, Context, Handler, ResponseFuture};
use dsp_domain::{dsp_message::DspMessage, dsp_message_result::DspMessageResult};

use crate::{command_processor::CommandProcessor, state::SharedState};

use super::user_actor_state::UserActorState;
struct UserActor{
    processor:CommandProcessor,
    state:SharedState
}

impl Actor for UserActor{
    type Context=Context<Self>;
    fn started(&mut self,_ctx:&mut Self::Context){
        
    }
    
    fn stopped(&mut self, ctx: &mut Self::Context) {
        
    }
}

impl Handler<DspMessage> for UserActor{
    type Result=ResponseFuture<Result<DspMessageResult,String>>;

    fn handle(&mut self, msg: DspMessage, ctx: &mut Self::Context) -> Self::Result {
        let c=Box::pin(async{
            let result=self.processor.process_command(msg,&mut self.state).await;
            result
        }); 
        c
    }
}