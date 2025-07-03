use async_trait::async_trait;
use domain::{
    dsp_message::DspMessage, envelope::Envelope, tracks_message_result::TracksMessageResult,
};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{
    command_dispatch::CommandDispatch,
    state::{TrackStoreProvider, TracksState},
};

pub struct ListDispatcher {}

#[async_trait]
impl CommandDispatch for ListDispatcher {
    async fn dispatch(
        &self,
        envelope: Envelope,
        state: &mut TracksState,
    ) -> Result<TracksMessageResult, String> {
        let result = match envelope.command {
            DspMessage::Ls { user_name } => self.internal_dispatch(user_name, state).await,
            _ => Err("".to_owned()),
        };
        return result;
    }
}

impl ListDispatcher {
    async fn internal_dispatch(
        &self,
        user_name: Option<String>,
        state: &mut TracksState,
    ) -> Result<TracksMessageResult, String> {
        let user_name = user_name.ok_or_else(|| "Invalid user_name")?;

        let tracks = state.get_all_tracks().await;
        Ok(TracksMessageResult {
            output: serde_json::to_string_pretty(&tracks).unwrap(),
            should_exit: false,
        })
    }
}
