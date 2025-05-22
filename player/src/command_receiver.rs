use std::sync::mpsc::Receiver;

use crate::player_command::PlayerCommand;

pub trait CommandReceiver {
    fn receive_command(&mut self) -> Result<PlayerCommand, String>;
}

pub struct LocalReceiver {
    pub receiver: Receiver<PlayerCommand>,
}

impl CommandReceiver for LocalReceiver {
    fn receive_command(&mut self) -> Result<PlayerCommand, String> {
        self.receiver.try_recv().map_err(|e| e.to_string())
    }
}
