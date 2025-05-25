use crate::player_command::PlayerMessage;

pub trait PlayerRef {
    fn send_message(&self, message: PlayerMessage) -> Result<(), String>;
}
