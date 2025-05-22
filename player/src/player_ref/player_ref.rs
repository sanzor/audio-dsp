use crate::player_command::PlayerCommand;

pub trait PlayerRef {
    fn send(&self, command: PlayerCommand) -> Result<(), String>;
}
