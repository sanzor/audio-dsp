use crate::{command_dispatch::CommandDispatch, state::TracksState};
use async_trait::async_trait;
use audiolib::audio_transform::AudioTransformMut;
use dsp_domain::{
    dsp_message::DspMessage, envelope::Envelope, tracks_message_result::TracksMessageResult,
};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct LowPassDispatcher {}

#[async_trait]
impl CommandDispatch for LowPassDispatcher {
    async fn dispatch(
        &self,
        envelope: Envelope,
        state: &mut TracksState,
    ) -> Result<TracksMessageResult, String> {
        match envelope.command {
            DspMessage::LowPass {
                user_name,
                track_name,
                cutoff,
            } => {
                self.internal_dispatch(user_name, track_name, cutoff, state)
                    .await
            }
            _ => Err("err".to_string()),
        }
    }
}

impl LowPassDispatcher {
    async fn internal_dispatch(
        &self,
        user_name: Option<String>,
        track_name: Option<String>,
        cutoff: f32,
        state: &mut TracksState,
    ) -> Result<TracksMessageResult, String> {
        let user_name = user_name.ok_or("Invalid name for user to high_pass on")?;
        let track_name = track_name.ok_or("Invalid name for track to high_pass on")?;
       
        let track_ref = state.get_track_ref_mut(track_name.as_str()).await?;
        let _ = track_ref.inner.data.low_pass_mut(cutoff);
        Ok(TracksMessageResult {
            output: format!("Normalize track {} succesful", track_name),
            should_exit: false,
        })
    }
}
