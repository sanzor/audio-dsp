use dsp_core::{command_processor::CommandProcessor, state::SharedState};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

use actix::{Actor, Addr, Context, Handler, ResponseFuture};

use dsp_domain::{
    audio_player_message::AudioPlayerMessage,
    audio_player_message_result::AudioPlayerMessageResult, dsp_message::DspMessage,
    tracks_message_result::TracksMessageResult,
};

use crate::audio_player_actor::audio_player_actor::AudioPlayerActor;
type Tracks = Arc<Mutex<SharedState>>;
type Players = Arc<Mutex<HashMap<String, Addr<AudioPlayerActor>>>>;
pub struct UserActor {
    processor: Arc<CommandProcessor>,
    track_state: Tracks,
    players: Players,
}

impl Actor for UserActor {
    type Context = Context<Self>;
}
impl UserActor {
    pub fn new(processor: Arc<CommandProcessor>, tracks: Tracks, players: Players) -> UserActor {
        UserActor {
            processor: processor,
            track_state: tracks,
            players: players,
        }
    }
}

impl Handler<DspMessage> for UserActor {
    type Result = ResponseFuture<Result<TracksMessageResult, String>>;

    // async move variant
    fn handle(&mut self, msg: DspMessage, ctx: &mut Self::Context) -> Self::Result {
        let proc = Arc::clone(&self.processor);

        // Lock the Mutex to access the mutable state
        let state_guard = Arc::clone(&self.track_state);
        Box::pin(async move { proc.process_crud_command(msg, state_guard).await })
    }
}

impl Handler<AudioPlayerMessage> for UserActor {
    type Result = ResponseFuture<Result<AudioPlayerMessageResult, String>>;
    fn handle(&mut self, msg: AudioPlayerMessage, ctx: &mut Self::Context) -> Self::Result {
        let players = Arc::clone(&self.players);
        let tracks = Arc::clone(&self.track_state);
        Box::pin(async move {
            match msg {
                AudioPlayerMessage::Play { track_id } => {
                    UserActor::handle_play(track_id, tracks, players).await
                }
                AudioPlayerMessage::Pause { track_id } => {
                    UserActor::handle_pause(track_id, tracks, players).await
                }
                AudioPlayerMessage::Stop { track_id } => {
                    UserActor::handle_stop(track_id, tracks, players).await
                }
            }
        })
    }
}
impl UserActor {
    pub async fn handle_play(
        track_id: Option<String>,
        tracks: Tracks,
        players: Players,
    ) -> Result<AudioPlayerMessageResult, String> {
        let track_id = track_id.ok_or_else(|| "invalid id".to_string())?;
        let guard = tracks.lock().await;
        let track_ref = guard.get_track_ref(&track_id).await?;
        let pl = players.lock().await;
        todo!()
    }
    pub async fn handle_pause(
        track_id: Option<String>,
        tracks: Tracks,
        players: Players,
    ) -> Result<AudioPlayerMessageResult, String> {
        let track_id = track_id.ok_or_else(|| "invalid id".to_string())?;
        let guard = tracks.lock().await;
        let track_ref = guard.get_track_ref(&track_id).await?;
        let pl = players.lock().await;
        todo!()
    }
    pub async fn handle_stop(
        track_id: Option<String>,
        tracks: Tracks,
        players: Players,
    ) -> Result<AudioPlayerMessageResult, String> {
        let track_id = track_id.ok_or_else(|| "invalid id".to_string())?;
        let guard = tracks.lock().await;
        let track_ref = guard.get_track_ref(&track_id).await?;
        let pl = players.lock().await;
        todo!()
    }
}
