use crate::{command_receiver::CommandReceiver, player_command::PlayerCommand};

pub struct TestReceiver {
    commands: Vec<PlayerCommand>,
}

impl TestReceiver {
    pub fn new(commands: Vec<PlayerCommand>) -> Self {
        Self { commands }
    }
}
impl CommandReceiver for TestReceiver {
    fn receive_command(&mut self) -> Result<PlayerCommand, String> {
        if self.commands.is_empty() {
            Err("No more commands".into())
        } else {
            Ok(self.commands.remove(0))
        }
    }
}
