use std::sync::{Arc, Mutex};

use crate::{command_dispatch::CommandDispatch, state::SharedState};
use async_trait::async_trait;
use audiolib::audio_transform::AudioTransformMut;
use dsp_domain::{
    dsp_message::DspMessage, dsp_message_result::DspMessageResult, envelope::Envelope,
};
pub(crate) struct HighPassDispatcher {}

#[async_trait]
impl CommandDispatch for HighPassDispatcher {
    async fn dispatch(
        &self,
        envelope: Envelope,
        state: Arc<Mutex<SharedState>>,
    ) -> Result<DspMessageResult, String> {
        match envelope.command {
            DspMessage::HighPass {
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

impl HighPassDispatcher {
    async fn internal_dispatch(
        &self,
        user_name: Option<String>,
        track_name: Option<String>,
        cutoff: f32,
        state: Arc<Mutex<SharedState>>,
    ) -> Result<DspMessageResult, String> {
        let user_name = user_name.ok_or("Invalid name for user to high_pass on")?;
        let track_name = track_name.ok_or("Invalid name for track to high_pass on")?;
        let track_ref = state.get_track_ref_mut(&user_name, &track_name).await?;
        let _ = track_ref.inner.data.high_pass_mut(cutoff);
        Ok(DspMessageResult {
            output: format!("Normalize track {} succesful", track_name),
            should_exit: false,
        })
    }
}
