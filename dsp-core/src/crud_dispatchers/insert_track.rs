use async_trait::async_trait;
use domain::{
    dsp_message::DspMessage, envelope::Envelope, track::Track,
    tracks_message_result::TracksMessageResult,
};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{command_dispatch::CommandDispatch, state::TracksState};

pub struct InsertTrackDispatcher {}
#[async_trait]
impl CommandDispatch for InsertTrackDispatcher {
    async fn dispatch(
        &self,
        envelope: Envelope,
        state: &mut TracksState,
    ) -> Result<TracksMessageResult, String> {
        match envelope.command {
            DspMessage::InsertTrack {
               track,
                user_id 
            } => {
                self.internal_dispatch(track, state)
                    .await
            }
            _ => Err("".to_owned()),
        }
    }
}

impl InsertTrackDispatcher {
    async fn internal_dispatch(
        &self,
        track: Track,
        state: &mut TracksState,
    ) -> Result<TracksMessageResult, String> {

        state.upsert_track(track).await?;

        Ok(TracksMessageResult {
            output: format!("Inserted track"),
            should_exit: false,
        })
    }
}
