
use crate::command_dispatchers::*;
use crate::{command::CommandResult, command_dispatch::CommandDispatch, envelope::Envelope};
use crate::state::SharedState;


pub enum DispatcherEnum{
    Load(LoadDispatcher),
    Info(InfoDispatcher),
    List(ListDispatcher),
    Save(UploadDispatcher),
    Copy(CopyDispatcher),
    Unload(UnloadDispatcher),
    Gain(GainDispatcher),
    Normalize(NormalizeDispatcher),
    LowPass(LowPassDispatcher),
    HighPass(HighPassDispatcher)
}

impl CommandDispatch for DispatcherEnum{
    fn dispatch(&self,envelope:Envelope,state: SharedState)->Result<CommandResult,String> {
       match self{
            DispatcherEnum::Info(handler)=>handler.dispatch(envelope,state),
            DispatcherEnum::List(handler)=>handler.dispatch(envelope, state)
       }
    }
}