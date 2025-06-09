use async_trait::async_trait;
use dsp_domain::{
    dsp_message::DspMessage, dsp_message_result::DspMessageResult, envelope::Envelope,
};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{command_dispatch::CommandDispatch, state::SharedState};

pub struct InfoDispatcher {}

#[async_trait]
impl CommandDispatch for InfoDispatcher {
    async fn dispatch(
        &self,
        envelope: Envelope,
        state: Arc<Mutex<SharedState>>,
    ) -> Result<DspMessageResult, String> {
        match envelope.command {
            DspMessage::Info {
                track_name,
                user_name,
            } => self.internal_dispatch(user_name, track_name, state).await,
            _ => Err("".to_owned()),
        }
    }
}

impl InfoDispatcher {
    async fn internal_dispatch(
        &self,
        user_name: Option<String>,
        track_name: Option<String>,
        state: Arc<Mutex<SharedState>>,
    ) -> Result<DspMessageResult, String> {
        let track_name = track_name.ok_or_else(|| "Invalid name")?;
        let user_name = user_name.ok_or_else(|| "Invalid user name")?;
        let mut state_guard = state.lock().await;
        let track_info = state_guard
            .get_track_info(&track_name)
            .await
            .map_err(|e| format!("Could not find track info for {}", e))?;
        Ok(DspMessageResult {
            output: serde_json::to_string_pretty(&track_info).unwrap(),
            should_exit: false,
        })
    }
}
