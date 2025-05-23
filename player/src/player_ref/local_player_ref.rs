use std::sync::mpsc::Sender;

use crate::player_command::PlayerMessage;

use super::player_ref::PlayerRef;

pub struct LocalPlayerRef {
    pub tx: Sender<PlayerMessage>,
}


impl PlayerRef for LocalPlayerRef {
    fn send_message(&self, message: PlayerMessage) -> Result<(), String> {
        self.tx
            .send(message)
            .map_err(|_| "Disconnected".to_string())
    }
}

