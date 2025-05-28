use dsp_domain::{dsp_command_result::DspCommandResult, envelope::Envelope};

use crate::state::{SharedState, State};


pub(crate) trait CommandDispatch {
    fn dispatch(&self, envelope: Envelope, state: SharedState) -> Result<DspCommandResult, String>;
}
