use std::{path::PathBuf, str::FromStr};
use dsp_domain::{command::{DspCommand, CommandResult}, envelope::Envelope};
use audiolib::audio_parse;

use crate::{

    command_dispatch::CommandDispatch,

    state::{SharedState, State},
};
pub(crate) struct UploadDispatcher {}

impl UploadDispatcher {
    fn internal_dispatch(
        &self,
        name: Option<String>,
        filename: Option<String>,
        state: &State,
    ) -> Result<CommandResult, String> {
        let name = name.ok_or_else(|| "Invalid name to upload track")?;
        let track_ref = state
            .get_track_ref(&name)
            .ok_or_else(|| "Could not find track_ref")?;
        let file_path_str = filename.unwrap_or_else(|| name.clone());
        let path = PathBuf::from_str(&file_path_str).map_err(|err| err.to_string())?;
        let _ = audio_parse::write_wav_file(&track_ref.inner.data, &path)?;
        Ok(CommandResult {
            output: format!(
                "Upload file successfully: {}",
                path.to_str().ok_or("invalid path")?.to_string()
            ),
        })
    }
}

impl CommandDispatch for UploadDispatcher {
    fn dispatch(&self, envelope: Envelope, state: SharedState) -> Result<CommandResult, String> {
        let result = state
            .try_write()
            .map_err(|e| e.to_string())
            .and_then(|mut guard| {
                let state_ref = &mut *guard;
                match envelope.command {
                    DspCommand::Upload { name, filename } => {
                        self.internal_dispatch(name, filename, state_ref)
                    }
                    _ => Err("".to_owned()),
                }
            });

        return result;
    }
}
