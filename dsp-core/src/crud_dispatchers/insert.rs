use async_trait::async_trait;
use dsp_domain::{
    dsp_message::DspMessage, envelope::Envelope, track::Track,
    tracks_message_result::TracksMessageResult,
};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{command_dispatch::CommandDispatch, state::SharedState};

pub struct InsertDispatcher {}
#[async_trait]
impl CommandDispatch for InsertDispatcher {
    async fn dispatch(
        &self,
        envelope: Envelope,
        state: Arc<Mutex<SharedState>>,
    ) -> Result<TracksMessageResult, String> {
        match envelope.command {
            DspMessage::Insert {
                user_name,
                track_payload,
            } => {
                self.internal_dispatch(user_name, track_payload, state)
                    .await
            }
            _ => Err("".to_owned()),
        }
    }
}

impl InsertDispatcher {
    async fn internal_dispatch(
        &self,
        user_name: Option<String>,
        track_payload: Option<String>,
        state: Arc<Mutex<SharedState>>,
    ) -> Result<TracksMessageResult, String> {
        let track_payload = track_payload.ok_or_else(|| "invalid payload".to_string())?;
        let track: Track = serde_json::from_str(&track_payload).unwrap();
        let user_name = user_name.ok_or_else(|| "invalid name".to_string())?;

        let mut state_guard = state.lock().await;
        state_guard.upsert_track(track).await?;

        Ok(TracksMessageResult {
            output: format!("Inserted track"),
            should_exit: false,
        })
    }
}
