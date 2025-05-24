use std::sync::mpsc::Receiver;

use crate::player_command::PlayerMessage;

pub trait CommandReceiver: Send {
    fn receive_message(&mut self) -> Result<PlayerMessage, String>;
}

pub struct LocalReceiver {
    pub receiver: Receiver<PlayerMessage>,
}

impl CommandReceiver for LocalReceiver {
    fn receive_message(&mut self) -> Result<PlayerMessage, String> {
        self.receiver.try_recv().map_err(|e| e.to_string())
    }
}
