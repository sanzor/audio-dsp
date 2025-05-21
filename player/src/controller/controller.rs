use crate::player_command::PlayerCommand;

pub trait Controller {
    fn send_command(&mut self, command: PlayerCommand);
    fn stop(&self);
}
