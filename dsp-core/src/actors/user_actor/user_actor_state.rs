use std::collections::HashMap;

use actix::Addr;
use dsp_domain::track::Track;

pub struct UserActorState{
    pub tracks:HashMap<String,Track>,
    pub players:HashMap<String,Addr<UserActorState>>
}

impl UserActorState{
    pub fn new()->UserActorState{
        UserActorState{tracks:HashMap::new(),players:HashMap::new()}
    }
}