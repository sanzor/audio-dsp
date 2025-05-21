use crate::player_command::PlayerCommand;

pub trait Controllable {
    fn handle_command(&mut self, command: PlayerCommand);
}
