use crate::{command_receiver::CommandReceiver, player_command::PlayerMessage};

pub struct TestReceiver {
    commands: Vec<PlayerMessage>,
}

impl TestReceiver {
    pub fn new(commands: Vec<PlayerMessage>) -> Self {
        Self { commands }
    }
}
impl CommandReceiver for TestReceiver {
    fn receive_message(&mut self) -> Result<PlayerMessage, String> {
        if self.commands.is_empty() {
            Err("No more commands".into())
        } else {
            Ok(self.commands.remove(0))
        }
    }
}
