use async_trait::async_trait;
use audiolib::audio_parse;
use dsp_domain::{
    dsp_message::DspMessage, dsp_message_result::DspMessageResult, envelope::Envelope,
};
use tokio::sync::Mutex;

use std::{path::PathBuf, str::FromStr, sync::Arc};

use crate::{command_dispatch::CommandDispatch, state::SharedState};
pub struct UploadDispatcher {}

#[async_trait]
impl CommandDispatch for UploadDispatcher {
    async fn dispatch(
        &self,
        envelope: Envelope,
        state: Arc<Mutex<SharedState>>,
    ) -> Result<DspMessageResult, String> {
        match envelope.command {
            DspMessage::Upload {
                user_name,
                track_name,
                filename,
            } => {
                self.internal_dispatch(user_name, track_name, filename, state)
                    .await
            }
            _ => Err("".to_owned()),
        }
    }
}

impl UploadDispatcher {
    async fn internal_dispatch(
        &self,
        user_name: Option<String>,
        track_name: Option<String>,
        filename: Option<String>,
        state: Arc<Mutex<SharedState>>,
    ) -> Result<DspMessageResult, String> {
        let track_name = track_name.ok_or_else(|| "Invalid name to upload track")?;
        let user_name = user_name.ok_or_else(|| "Invalid name to upload track")?;
        let state_guard = state.lock().await;
        let track_ref = state_guard.get_track_ref(&track_name).await?;
        let file_path_str = filename.unwrap_or_else(|| track_name.clone());
        let path = PathBuf::from_str(&file_path_str).map_err(|err| err.to_string())?;
        let _ = audio_parse::write_wav_file(&track_ref.inner.data, &path)?;
        Ok(DspMessageResult {
            output: format!(
                "Upload file successfully: {}",
                path.to_str().ok_or("invalid path")?.to_string()
            ),
            should_exit: false,
        })
    }
}
