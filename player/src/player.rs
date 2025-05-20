use std::sync::mpsc::Receiver;

use dsp_domain::track::Track;

use crate::{player_command::PlayerCommand, player_params::PlayerParams};


pub struct Player{
    track:Track,
    cursor:usize,
    is_playing:bool
}


impl Player{
    pub fn new(player_params:PlayerParams)->Player{
        Player{track:player_params.track,is_playing:false,cursor:0}
    }

    pub fn run(&mut self,cmd_rx:Receiver<PlayerCommand>){
        loop{
            while let Ok(cmd)=cmd_rx.try_recv(){
                self.handle_command(cmd);
            }
            if self.is_playing{
                self.play_frame();
                self.cursor+=1;
            }
        }
    }
    fn handle_command(&mut self,cmd:PlayerCommand){

    }
    fn play_frame(&mut self){
        match self.track.data.channels{
                Channels
        }
    }
    fn get_stereo_frame(&self,cursor:usize)->Option<(f32,f32)>{
        let i=cursor*2;
        if i+1>=self.track.data.samples.len(){
            return None
        }
        Some((self.track.data.samples[i],self.track.data.samples[i+1]))
    }

    fn get_mono_sample(&self,cursor:usize)->Option<f32,f32>{
        self.track.data.samples.get(cursor).copied()
    }
}