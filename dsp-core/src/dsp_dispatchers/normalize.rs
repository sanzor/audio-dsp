use crate::{command_dispatch::CommandDispatch, state::SharedState};
use async_trait::async_trait;
use audiolib::audio_transform::AudioTransformMut;
use dsp_domain::{dsp_message_result::DspMessageResult, envelope::Envelope, dsp_message::DspMessage};

pub(crate) struct NormalizeDispatcher {}

#[async_trait]
impl CommandDispatch for NormalizeDispatcher {
    async fn dispatch_mut(
        &self,
        envelope: Envelope,
        state: &mut SharedState,
    ) -> Result<DspMessageResult, String> {
        match envelope.command {
            DspMessage::Normalize {
                user_name,
                track_name,
                mode: _,
                parallelism: _,
            } => self.internal_dispatch(user_name, track_name, state).await,
            _ => Err("err".to_string()),
        }
    }
}

impl NormalizeDispatcher {
    async fn internal_dispatch(
        &self,
        user_name: Option<String>,
        track_name: Option<String>,
        state: &mut SharedState,
    ) -> Result<DspMessageResult, String> {
        let user_name = user_name.ok_or("Invalid name for user to perform normalize on")?;
        let track_name = track_name.ok_or("Invalid name for track to perform normalize on")?;
        let track_ref = state.get_track_ref_mut(&user_name, &track_name).await?;
        let _ = track_ref.inner.data.normalize_mut();
        Ok(DspMessageResult {
            output: format!("Normalize track {} succesful", track_name),
            should_exit: false,
        })
    }
}
