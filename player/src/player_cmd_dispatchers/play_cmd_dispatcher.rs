use crate::player_command::PlayerCommand;

use super::PlayerCommandDispatch;

pub struct PlayCommandDispatcher{}


impl PlayerCommandDispatch for PlayCommandDispatcher{
    fn dispatch(command:PlayerCommand)->Result<(),String> {
        todo!()
    }
}