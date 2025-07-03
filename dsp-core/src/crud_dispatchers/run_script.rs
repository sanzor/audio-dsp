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

pub struct RunScriptDispatcher {}

#[async_trait]
impl CommandDispatch for RunScriptDispatcher {
    async fn dispatch(
        &self,
        envelope: Envelope,
        state: &mut TracksState,
    ) -> Result<TracksMessageResult, String> {
        match envelope.command {
            DspMessage::Copy {
                user_name,
                track_name,
                copy_name,
            } => {
                self.internal_dispatch(user_name, track_name, copy_name, state)
                    .await
            }
            _ => Err("Could not perform copy".to_owned()),
        }
    }
}

impl RunScriptDispatcher {
    async fn internal_dispatch(
        &self,
        user_name: Option<String>,
        name: Option<String>,
        copy_name: Option<String>,
        state: &mut TracksState,
    ) -> Result<TracksMessageResult, String> {
        let track_name = name.ok_or("Invalid name for copy")?;
        let user_name = user_name.ok_or("Invalid name for copy")?;

        let mut new_track = state.get_track_copy(&track_name.clone()).await?;

        let copy_name = copy_name.unwrap_or_else(|| new_track.info.name.clone() + "v2");
        new_track.info.name = copy_name.clone();
        let _ = state.upsert_track(new_track);
        Ok(TracksMessageResult {
            output: format!("Copied successfully track:{} to {}", track_name, copy_name),
            should_exit: false,
        })
    }
}
