

use dsp_domain::{command::CommandResult, envelope::Envelope};

use crate::state::SharedState;
pub(crate) trait CommandDispatch {
    fn dispatch(&self, envelope: Envelope, state: SharedState) -> Result<CommandResult, String>;
}
