use crate::{
    command_dispatch::CommandDispatch,
    state::{SharedState, State},
};
use dsp_domain::{
    dsp_command::DspCommand, dsp_command_result::DspCommandResult, envelope::Envelope,
};
pub(crate) struct DeleteDispatcher {}

impl CommandDispatch for DeleteDispatcher {
    fn dispatch(&self, envelope: Envelope, state: SharedState) -> Result<DspCommandResult, String> {
        let mut guard = state.try_write().map_err(|e| e.to_string())?;
        let state = &mut *guard;
        match envelope.command {
            DspCommand::Delete { name } => self.internal_dispatch(name, state),
            _ => Err("".to_string()),
        }
    }
}
impl DeleteDispatcher {
    fn internal_dispatch(
        &self,
        name: Option<String>,
        state: &mut State,
    ) -> Result<DspCommandResult, String> {
        let name = name.ok_or_else(|| "Invalid name for deleted track".to_string())?;
        let _ = state.delete_track(&name)?;
        Ok(DspCommandResult {
            output: format!("Delete track {} succesful", name),
            should_exit: false,
        })
    }
}
