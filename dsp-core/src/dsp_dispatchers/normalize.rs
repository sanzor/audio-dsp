use crate::{command_dispatch::CommandDispatch, state::SharedState};
use async_trait::async_trait;
use audiolib::audio_transform::AudioTransformMut;
use dsp_domain::{dsp_message::DspMessage, tracks_message_result::TracksMessageResult, envelope::Envelope};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct NormalizeDispatcher {}

#[async_trait]
impl CommandDispatch for NormalizeDispatcher {
    async fn dispatch(
        &self,
        envelope: Envelope,
        state: Arc<Mutex<SharedState>>,
    ) -> Result<TracksMessageResult, String> {
        match envelope.command {
            DspMessage::Normalize {
                user_name,
                track_name,
                mode: _,
                parallelism: _,
            } => self.internal_dispatch(track_name, state).await,
            _ => Err("err".to_string()),
        }
    }
}

impl NormalizeDispatcher {
    async fn internal_dispatch(
        &self,
        track_name: Option<String>,
        state: Arc<Mutex<SharedState>>,
    ) -> Result<TracksMessageResult, String> {
        let track_name = track_name.ok_or("Invalid name for track to perform normalize on")?;
        let state = &mut *state.lock().await;
        let mut_ref = state.get_track_ref_mut(track_name.as_str()).await?;
        mut_ref.inner.data.normalize_mut();
        Ok(TracksMessageResult {
            output: format!("Normalize track {} succesful", track_name),
            should_exit: false,
        })
    }
}
