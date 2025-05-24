use std::sync::mpsc::Sender;

use crate::player_core::player_state::PlayerState;

pub enum PlayerCommand {
    Play,
    Pause,
    Seek(usize),
    Stop,
    Close,
}
pub enum QueryMessage {
    GetState{to:Option<Sender<Result<QueryResult,String>>>},
}
pub enum PlayerMessage {
    Command { command: PlayerCommand },
    Query { query: QueryMessage },
}

pub enum QueryResult{
    State{state:PlayerState}
}
