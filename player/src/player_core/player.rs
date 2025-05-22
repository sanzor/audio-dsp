use std::sync::mpsc::{channel, Receiver, Sender};

use audiolib::Channels;
use dsp_domain::track::Track;

use crate::{
    audio_sink::AudioSink, command_receiver::CommandReceiver, player_command::PlayerCommand,
    player_params::PlayerParams, AudioFrame,
};

use super::{player_effect::PlayerEffect, player_state::PlayerState};
pub struct Player<S>
where
    S: AudioSink,
{
    track: Track,
    cursor: usize,
    state: PlayerState,
    is_playing: bool,
    sink: S,
    receiver: Box<dyn CommandReceiver + Send>,
    self_sender: Sender<PlayerCommand>,
    self_receiver: Receiver<PlayerCommand>,
}

impl<S: AudioSink> Player<S> {
    pub fn new(
        player_params: PlayerParams,
        audio_sink: S,
        receiver: Box<dyn CommandReceiver + Send>,
    ) -> Player<S> {
        let (tx, rx) = channel();
        Player {
            track: player_params.track,
            is_playing: false,
            cursor: 0,
            sink: audio_sink,
            receiver: receiver,
            state: PlayerState::Stopped,
            self_sender: tx,
            self_receiver: rx,
        }
    }
    pub fn cursor_position(&self) -> usize {
        self.cursor
    }
    pub fn player_state(&self) -> PlayerState {
        self.state
    }
    pub fn run(&mut self) -> Result<(), String> {
        loop {
            while let Ok(cmd) = self.self_receiver.try_recv() {
                self.handle_self_command(cmd);
            }
            while let Ok(cmd) = self.receiver.receive_command() {
                if let Some(effect) = self.handle_command(cmd) {
                    match effect {
                        PlayerEffect::Close => return Ok(()),
                        PlayerEffect::ResetCursor => {
                            self.cursor = 0;
                        }
                        PlayerEffect::Seek { pos: pos } => self.cursor = pos,
                        PlayerEffect::Schedule { command: command } => {
                            self.self_sender.send(command).map_err(|e| e.to_string())?
                        }
                    }
                }
            }
            match self.state {
                PlayerState::Paused | PlayerState::Stopped => {}
                PlayerState::Playing => {
                    if let Some(frame) = self.get_frame(self.cursor) {
                        self.sink.write_frame(frame)?;
                        self.cursor += 1;
                    } else {
                        self.state = PlayerState::Paused;
                    }
                }
            }
        }
    }
    fn handle_command(&mut self, cmd: PlayerCommand) -> Option<PlayerEffect> {
        let (new_state, effect) = self.transition(self.state, cmd);
        self.state = new_state;
        effect
    }
    fn handle_self_command(&mut self, cmd: PlayerCommand) -> Option<PlayerEffect> {
        None
    }

    fn transition(
        &self,
        state: PlayerState,
        cmd: PlayerCommand,
    ) -> (PlayerState, Option<PlayerEffect>) {
        match (state, cmd) {
            (PlayerState::Playing, PlayerCommand::Pause) => (PlayerState::Paused, None),
            (_, PlayerCommand::Stop) => (PlayerState::Stopped, Some(PlayerEffect::ResetCursor)),
            (_, PlayerCommand::Seek(pos)) => {
                (PlayerState::Stopped, Some(PlayerEffect::Seek { pos: pos }))
            }
            (PlayerState::Paused | PlayerState::Stopped, PlayerCommand::Play) => {
                (PlayerState::Playing, None)
            }
            (_, PlayerCommand::Close) => (PlayerState::Stopped, Some(PlayerEffect::Close)),
            _ => (state, None),
        }
    }

    fn get_frame(&self, cursor: usize) -> Option<AudioFrame> {
        let frame: Option<Vec<f32>> = match self.track.data.channels {
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
