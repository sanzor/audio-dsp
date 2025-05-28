use crate::{command_dispatch::CommandDispatch, state::SharedState};
    use dsp_domain::{dsp_command_result::DspCommandResult, message::Message};
pub(crate) struct PauseDispatcher{}



impl CommandDispatch for PauseDispatcher{
    fn dispatch(&self, envelope: dsp_domain::envelope::Envelope, state: crate::state::SharedState) -> Result<dsp_domain::dsp_command_result::DspCommandResult, String> {
        match envelope.command{
            Message::Pause { id }=>self.internal_dispatch(state,id),
            _ => Err("Invalid pause command".to_string())
        }
    }
}
impl PauseDispatcher{
    fn internal_dispatch(&self,state:SharedState,id:Option<String>)->Result<DspCommandResult,String>{
        let name=id.ok_or_else(||"invalid name".to_string())?;
        let player=state.try_write()?.
    }
}