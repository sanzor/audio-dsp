use std::{collections::HashMap, sync::{Arc, RwLock}};

use cli_app::dispatcher_enum::DispatcherEnum;

use crate::{command_dispatchers::CommandDispatcher, state::State,scheduler::SharedState};

pub struct Scheduler{
    pub dispatch_map:HashMap<String,DispatcherEnum>,
    pub state:SharedState
}