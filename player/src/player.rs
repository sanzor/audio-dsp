use std::sync::mpsc::Receiver;

use audiolib::Channels;
use dsp_domain::track::Track;

use crate::{audio_sink::AudioSink, player_command::PlayerCommand, player_params::PlayerParams, AudioFrame};

pub struct Player<S:AudioSink> {
    track: Track,
    cursor: usize,
    is_playing: bool,
    sink:S
}

impl<S:AudioSink> Player<S> {
    pub fn new(player_params: PlayerParams,audio_sink:S) -> Player<S> {
        Player {
            track: player_params.track,
            is_playing: false,
            cursor: 0,
            sink:audio_sink
        }
    }

    pub fn run(&mut self, cmd_rx: Receiver<PlayerCommand>) {
        loop {
            while let Ok(cmd) = cmd_rx.try_recv() {
                self.handle_command(cmd);
            }
            if self.is_playing {
                if let Some(frame)=self.get_frame(self.cursor){
                    self.sink.write_frame(frame);
                    self.cursor+=1;
                }
                else{
                    self.is_playing=false
                }
            }
        }
    }
    fn handle_command(&mut self, cmd: PlayerCommand) {}
    fn play_frame(&mut self) {
        if let Some(frame)=self.get_frame(self.cursor){
            self.sink.write_frame(frame);
        }
        else {self.is_playing=false;}

    }

    fn get_frame(&self, cursor: usize) -> Option<AudioFrame> {
        let frame:Option<Vec<f32>> = match self.track.data.channels {
            Channels::Mono => self.track.data.samples.get(cursor).map(|&s| vec![s]),
            Channels::Stereo => {
                let i = cursor * 2;
                if i + 1 >= self.track.data.samples.len() {
                    None
                } else {
                    Some(vec![
                        self.track.data.samples[i],
                        self.track.data.samples[i + 1],
                    ])
                }
            }
        };
        frame
    }
}
