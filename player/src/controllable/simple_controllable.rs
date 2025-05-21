use crate::{audio_sink::{AudioSink, StdSink}, player::Player};

use super::Controllable;

pub struct SimpleControllable<S:AudioSink> {
    player: Player<S>,


}

impl<S:AudioSink> Controllable for SimpleControllable<S> {
    fn handle_command(&mut self, command: crate::player_command::PlayerCommand) {
        self.player.
    }
}

impl SimpleControllable{
    pub fn new(){
        
    }
}
