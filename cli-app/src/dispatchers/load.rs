use std::path::PathBuf;

use crate::{
    command::{Command, CommandResult},
    command_dispatch::CommandDispatch,
    envelope::Envelope,
    state::{SharedState, State},
    track::{Track, TrackInfo},
};

pub struct LoadDispatcher {}

impl CommandDispatch for LoadDispatcher {
    fn dispatch(&self, envelope: Envelope, state: SharedState) -> Result<CommandResult, String> {
        let result = state
            .try_write()
            .map_err(|e| e.to_string())
            .and_then(|mut guard| {
                let state_ref = &mut *guard;
                match envelope.command {
                    Command::Load { name, filename } => {
                        self.internal_dispatch(name, filename, state_ref)
                    }
                    _ => Err("".to_owned()),
                }
            });

        return result;
    }
}

impl LoadDispatcher {
    fn internal_dispatch(
        &self,
        name: Option<String>,
        filename: Option<String>,
        state: &mut State,
    ) -> Result<CommandResult, String> {
        let filename = filename.ok_or_else(|| "Invalid file name".to_string())?;
        let filepath = PathBuf::from(&filename);
        let name = name.unwrap_or_else(|| filename.clone());

        let audio_buffer = audiolib::audio_parse::read_wav_file(&filepath)?;
        let new_track = Track {
            info: TrackInfo { name: name.clone() },
            data: audio_buffer,
        };

        state.upsert_track(new_track)?;

        Ok(CommandResult {
            output: format!("Loaded track '{}' from '{}'", name, filepath.display()),
        })
    }
}
