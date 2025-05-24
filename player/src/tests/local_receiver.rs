use std::sync::mpsc::Receiver;

use crate::{command_receiver::CommandReceiver, player_command::PlayerMessage};

pub struct LocalReceiver {
    pub inner: Receiver<PlayerMessage>,
}

impl CommandReceiver for LocalReceiver {
    fn receive_message(&mut self) -> Result<PlayerMessage, String> {
        self.inner.try_recv().map_err(|e| e.to_string())
    }
}
