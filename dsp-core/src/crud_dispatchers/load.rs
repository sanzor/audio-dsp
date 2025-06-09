use async_trait::async_trait;
use dsp_domain::{
    dsp_message::DspMessage,
    dsp_message_result::DspMessageResult,
    envelope::Envelope,
    track::{Track, TrackInfo},
};
use std::{path::PathBuf, sync::Arc};
use tokio::sync::Mutex;

use crate::{command_dispatch::CommandDispatch, state::SharedState};

pub struct LoadDispatcher {}
#[async_trait]
impl CommandDispatch for LoadDispatcher {
    async fn dispatch(
        &self,
        envelope: Envelope,
        state: Arc<Mutex<SharedState>>,
    ) -> Result<DspMessageResult, String> {
        match envelope.command {
            DspMessage::Load {
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

impl LoadDispatcher {
    async fn internal_dispatch(
        &self,
        user_name: Option<String>,
        track_name: Option<String>,
        filename: Option<String>,
        state: Arc<Mutex<SharedState>>,
    ) -> Result<DspMessageResult, String> {
        let filename = filename.ok_or_else(|| "Invalid file name".to_string())?;
        let filepath = PathBuf::from(&filename);
        let track_name = track_name.unwrap_or_else(|| filename.clone());
        let user_name = user_name.unwrap_or_else(|| filename.clone());

        let audio_buffer = audiolib::audio_parse::read_wav_file(&filepath)?;
        let new_track = Track {
            info: TrackInfo {
                name: track_name.clone(),
            },
            data: audio_buffer,
        };
        let mut state_guard = state.lock().await;
        state_guard.upsert_track(new_track).await?;

        Ok(DspMessageResult {
            output: format!(
                "Loaded track '{}' from '{}'",
                track_name,
                filepath.display()
            ),
            should_exit: false,
        })
    }
}
