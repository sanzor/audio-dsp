use std::collections::HashMap;

use cli_app::dispatcher_enum::DispatcherEnum;

use crate::state::{SharedState,create_shared_state};


pub struct Scheduler{
    pub dispatch_map:HashMap<String,DispatcherEnum>,
    state:SharedState
}

impl Scheduler{
    pub fn new()->Scheduler{
        Scheduler { dispatch_map:Self::create_dispatch_map() , state: create_shared_state() }
    }
    fn create_dispatch_map()->HashMap<String,DispatcherEnum>{
        HashMap::new()
    }

}