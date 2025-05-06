

use crate::{command::CommandResult, envelope::Envelope};
use crate::state::SharedState;
pub trait CommandDispatch{
    fn dispatch(&self,envelope:Envelope,state: SharedState)->Result<CommandResult,String>;
}