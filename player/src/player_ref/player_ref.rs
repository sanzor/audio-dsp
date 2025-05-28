use crate::player_command::PlayerMessage;

pub trait PlayerRef: Send + Sync {
    fn id(&self) -> &str;
    fn send_message(&self, message: PlayerMessage) -> Result<(), String>;
}
