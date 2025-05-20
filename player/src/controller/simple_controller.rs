use crate::player_command::PlayerCommand;

use super::Controller;

pub struct SimpleController{

}

impl Controller for SimpleController{
    fn send_command(&mut self,command:PlayerCommand) {
        todo!()
    }

    fn stop(&self) {
        todo!()
    }
}