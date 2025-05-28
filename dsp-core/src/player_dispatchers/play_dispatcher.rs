use crate::command_dispatch::CommandDispatch;
use dsp_domain::{dsp_command_result::DspCommandResult, message::Message};
pub(crate) struct PlayDispatcher{}

impl CommandDispatch for PlayDispatcher{
    fn dispatch(&self, envelope: dsp_domain::envelope::Envelope, state: crate::state::SharedState) -> Result<dsp_domain::dsp_command_result::DspCommandResult, String> {
        match envelope.command{
            Message::
        }
    }
}
impl PlayDispatcher{
    fn internal_dispatch(&mut self,){

    }
}