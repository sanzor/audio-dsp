use crate::{command_dispatch::CommandDispatch, state::SharedState};
use async_trait::async_trait;
use dsp_domain::{dsp_command_result::DspCommandResult, envelope::Envelope, message::Message};
pub(crate) struct DeleteDispatcher {}

#[async_trait]
impl CommandDispatch for DeleteDispatcher {
    async fn dispatch(
        &self,
        envelope: Envelope,
        state: SharedState,
    ) -> Result<DspCommandResult, String> {
        match envelope.command {
            Message::Delete { track_name } => self.internal_dispatch(name, state).await,
            _ => Err("".to_string()),
        }
    }
}
impl DeleteDispatcher {
    async fn internal_dispatch(
        &self,
        name: Option<String>,
        state: SharedState,
    ) -> Result<DspCommandResult, String> {
        let name = name.ok_or_else(|| "Invalid name for deleted track".to_string())?;
        let _ = state.delete_track(&name).await?;
        Ok(DspCommandResult {
            output: format!("Delete track {} succesful", &name),
            should_exit: false,
        })
    }
}
