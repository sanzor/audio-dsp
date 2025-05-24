use crate::player_command::{PlayerCommand, PlayerMessage, QueryResult};

pub trait PlayerRef {
    fn send_message(&self, message: PlayerMessage) -> Result<(), String>;
}