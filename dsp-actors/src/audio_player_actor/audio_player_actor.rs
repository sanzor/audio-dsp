use std::sync::Arc;

use actix::{Actor, Context, Handler, ResponseFuture};
use dsp_domain::track::Track;
use player::audio_sink::AudioSink;
use tokio::sync::Mutex;

use crate::{AudioPlayerMessage, AudioPlayerResult};
enum AudioPlayerState {
    Paused,
    Playing,
}
pub struct AudioPlayerActor {
    pub state: State,
}
pub struct AudioPlayerInternalState {
    sink: Box<dyn AudioSink>,
    state: AudioPlayerState,
    cursor: usize,
    track: Track,
}
type State = Arc<Mutex<AudioPlayerInternalState>>;
impl Actor for AudioPlayerActor {
    type Context = Context<Self>;
}
impl AudioPlayerActor {
    pub fn new(track: &Track, sink: Box<dyn AudioSink>) -> AudioPlayerActor {
        let state = AudioPlayerInternalState {
            sink: sink,
            track: track.clone(),
            cursor: 0,
            state: AudioPlayerState::Paused,
        };
        AudioPlayerActor {
            state: Arc::new(Mutex::new(state)),
        }
    }
}
impl Handler<AudioPlayerMessage> for AudioPlayerActor {
    type Result = ResponseFuture<Result<AudioPlayerResult, String>>;

    fn handle(&mut self, msg: AudioPlayerMessage, ctx: &mut Self::Context) -> Self::Result {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let mut s = state.lock().await;
            match msg {
                AudioPlayerMessage::Play => {
                    if matches!(s.state, AudioPlayerState::Playing) {
                        s.state = AudioPlayerState::Playing;
                    }
                    Ok(AudioPlayerResult {})
                }
                AudioPlayerMessage::Pause => {
                    if matches!(s.state, AudioPlayerState::Playing) {
                        s.state = AudioPlayerState::Paused;
                    }
                    Ok(AudioPlayerResult {})
                }
                AudioPlayerMessage::Stop => {
                    s.state = AudioPlayerState::Paused;
                    s.cursor = 0;
                    Ok(AudioPlayerResult {})
                }
                AudioPlayerMessage::Seek { position } => {
                    s.cursor = position as usize;
                    Ok(AudioPlayerResult {})
                }
            }
        })
    }
}
impl AudioPlayerActor {
    pub async fn handle_msg(
        state: State,
        msg: AudioPlayerMessage,
    ) -> Result<AudioPlayerResult, String> {
        let rez = match msg {
            AudioPlayerMessage::Play => AudioPlayerActor::handle_play(Arc::clone(&state)).await,
            AudioPlayerMessage::Pause => AudioPlayerActor::handle_pause(Arc::clone(&state)).await,
            AudioPlayerMessage::Stop => AudioPlayerActor::handle_stop(Arc::clone(&state)).await,
            AudioPlayerMessage::Seek { position } => {
                AudioPlayerActor::handle_seek(Arc::clone(&state), position).await
            }
        };
        Ok(AudioPlayerResult {})
    }
    pub async fn handle_play(state: State) -> Result<(), String> {
        todo!()
    }
    pub async fn handle_pause(state: State) -> Result<(), String> {
        todo!()
    }
    pub async fn handle_seek(state: State, position: u32) -> Result<(), String> {
        todo!()
    }
    pub async fn handle_stop(state: State) -> Result<(), String> {
        todo!()
    }
}
