use std::{collections::HashMap, sync::Arc};

use actix::Actor;
use dsp_core::{command_processor::CommandProcessor, state::SharedState};
use tokio::sync::Mutex;

use crate::user_actor::user_actor::UserActor;

fn create_actor()->UserActor{
    let processor=Arc::new(CommandProcessor::create_processor());
    let tracks=Arc::new(Mutex::new(SharedState::new()));
    let players=Arc::new(Mutex::new(HashMap::new()));
    UserActor::new(processor, tracks, players)
}
#[actix::test]
async fn can_run_load(){
    let addr=create_actor().start();
    let 
}