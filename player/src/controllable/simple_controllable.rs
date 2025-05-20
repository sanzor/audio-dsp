use crate::player::Player;

use super::Controllable;

pub struct SimpleControllable{
    player:Player
}

impl Controllable for SimpleControllable{
    fn handle_command(&mut self,command:crate::player_command::PlayerCommand) {
        
    }
}