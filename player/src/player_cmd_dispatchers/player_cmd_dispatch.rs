use crate::player_command::PlayerCommand;

pub trait PlayerCommandDispatch{
     fn dispatch(command:PlayerCommand)->Result<(),String>;
}