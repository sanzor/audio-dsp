use std::sync::{
    mpsc::{channel, Receiver, Sender},
    Arc, Mutex,
};

use audiolib::Channels;
use dsp_domain::track::Track;

use crate::{
    audio_sink::{self, AudioSink},
    command_receiver::CommandReceiver,
    player_command::{PlayerCommand, PlayerMessage, QueryMessage, QueryResult},
    player_params::PlayerParams,
    AudioFrame,
};

use super::{player_effect::PlayerEffect, player_state::PlayerState, player_states::PlayerStates};
pub struct Player<S>
where
    S: AudioSink,
{
    track: Track,
    state: Arc<Mutex<PlayerState>>,
    sink: S,
    receiver: Box<dyn CommandReceiver + Send>,
    self_sender: Sender<PlayerMessage>,
    self_receiver: Receiver<PlayerMessage>,
}

impl<S: AudioSink> Player<S> {
    pub fn new(
        player_params: PlayerParams,
        sink: S,
        message_receiver: Box<dyn CommandReceiver + Send>,
    ) -> Player<S> {
        let (tx, rx) = channel();
        Player {
            track: player_params.track,
            state: Arc::new(Mutex::new(PlayerState {
                current_state: PlayerStates::Stopped,
                cursor: 0,
            })),
            sink,
            receiver: message_receiver,
            self_sender: tx,
            self_receiver: rx,
        }
    }

    pub fn run(&mut self) -> Result<(), String> {
        loop {
            while let Ok(self_message) = self.self_receiver.try_recv() {
                self.handle_self_message(self_message);
            }
            // let mut state=self.state.try_lock().map_err(|e|e.to_string())?;
            while let Ok(message) = self.receiver.receive_message() {
                match message {
                    PlayerMessage::Query { query } => {let _ = self.handle_query(query);},
                    PlayerMessage::Command { command: command } => {
                        if let Some(effect) = self.handle_command(command) {
                            let mut state = self.state.try_lock().map_err(|e| e.to_string())?;
                            match effect {
                                PlayerEffect::Close => return Ok(()),
                                PlayerEffect::ResetCursor => {
                                    state.cursor = 0;
                                }
                                PlayerEffect::Seek { pos: pos } => {
                                    state.cursor = pos;
                                }
                                PlayerEffect::Schedule { command: command } => self
                                    .self_sender
                                    .send(PlayerMessage::Command { command: command })
                                    .map_err(|e| e.to_string())?,
                            }
                        }
                    }
                }
            }
            let mut st = self.state.try_lock().map_err(|e| e.to_string())?;
            match st.current_state {
                PlayerStates::Paused | PlayerStates::Stopped => {}
                PlayerStates::Playing => {
                    if let Some(frame) = self.get_frame(st.cursor) {
                        self.sink.write_frame(frame)?;
                        st.cursor += 1;
                    } else {
                        st.current_state = PlayerStates::Paused;
                    }
                }
            }
        }
    }
    pub fn query(&self,query: QueryMessage)->Result<(),String> {
      match query{
                QueryMessage::GetState { to:to }=>{
                    let message_to_send=self.state.try_lock().map_err(|e|e.to_string())?.clone();
                    let result=Ok(QueryResult::State { state: message_to_send });
                    let sender=to.ok_or_else(||"err".to_string())?;
                    let r=sender.send(result);
                    Ok(())
                }
                _=>Err("Not supported".to_string())
        }
    }
    fn handle_query(&self, query: QueryMessage)->Result<(),String> {
        match query{
                QueryMessage::GetState { to:to }=>{
                    let message_to_send=self.state.try_lock().map_err(|e|e.to_string())?.clone();
                    let result=Ok(QueryResult::State { state: message_to_send });
                    let sender=to.ok_or_else(||"err".to_string())?;
                    sender.send(result);
                    Ok(())
                }
                _=>Err("Not supported".to_string())
        }
    }
    fn handle_command(&mut self, cmd: PlayerCommand) -> Option<PlayerEffect> {
        let mut st = self.state.try_lock().ok()?;
        let (new_state, effect) = self.transition(st.current_state, cmd);
        st.current_state = new_state;
        effect
    }
    fn handle_self_message(&mut self, cmd: PlayerMessage) -> Option<PlayerEffect> {
        None
    }

    fn transition(
        &self,
        state: PlayerStates,
        cmd: PlayerCommand,
    ) -> (PlayerStates, Option<PlayerEffect>) {
        match (state, cmd) {
            (PlayerStates::Playing, PlayerCommand::Pause) => (PlayerStates::Paused, None),
            (_, PlayerCommand::Stop) => (PlayerStates::Stopped, Some(PlayerEffect::ResetCursor)),
            (_, PlayerCommand::Seek(pos)) => {
                (PlayerStates::Stopped, Some(PlayerEffect::Seek { pos: pos }))
            }
            (PlayerStates::Paused | PlayerStates::Stopped, PlayerCommand::Play) => {
                (PlayerStates::Playing, None)
            }
            (_, PlayerCommand::Close) => (PlayerStates::Stopped, Some(PlayerEffect::Close)),
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
