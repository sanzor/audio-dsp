use std::sync::mpsc::Sender;

use crate::player_command::PlayerCommand;

use super::player_ref::PlayerRef;

pub struct LocalPlayerRef {
    tx: Sender<PlayerCommand>,
}

impl PlayerRef for LocalPlayerRef {
    fn send(&self, command: PlayerCommand) -> Result<(), String> {
        self.tx
            .send(command)
            .map_err(|_| "Disconnected".to_string())
    }
}
impl LocalPlayerRef {
    pub fn new(tx: Sender<PlayerCommand>) -> LocalPlayerRef {
        LocalPlayerRef { tx: tx }
    }
}
