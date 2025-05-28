use dsp_domain::{dsp_command_result::DspCommandResult, envelope::Envelope};

use crate::command_dispatch::CommandDispatch;
use crate::crud_dispatchers::{
    CopyDispatcher, DeleteDispatcher, InfoDispatcher, ListDispatcher, LoadDispatcher,
    RunScriptDispatcher, UploadDispatcher,
};
use crate::dsp_dispatchers::{
    GainDispatcher, HighPassDispatcher, LowPassDispatcher, NormalizeDispatcher,
};
use crate::player_dispatchers::{PauseDispatcher, PlayDispatcher, StopDispatcher};
use crate::state::SharedState;

pub(crate) enum DispatcherEnum {
    Load(LoadDispatcher),
    Info(InfoDispatcher),
    List(ListDispatcher),
    Upload(UploadDispatcher),
    Copy(CopyDispatcher),
    Delete(DeleteDispatcher),
    Gain(GainDispatcher),
    Normalize(NormalizeDispatcher),
    LowPass(LowPassDispatcher),
    HighPass(HighPassDispatcher),
    RunScript(RunScriptDispatcher),
    Play(PlayDispatcher),
    Pause(PauseDispatcher),
    Stop(StopDispatcher),
    
}

impl CommandDispatch for DispatcherEnum {
    fn dispatch(&self, envelope: Envelope, state: SharedState) -> Result<DspCommandResult, String> {
        match self {
            DispatcherEnum::Load(handler) => handler.dispatch(envelope, state),
            DispatcherEnum::Info(handler) => handler.dispatch(envelope, state),
            DispatcherEnum::List(handler) => handler.dispatch(envelope, state),
            DispatcherEnum::Upload(handler) => handler.dispatch(envelope, state),
            DispatcherEnum::Copy(handler) => handler.dispatch(envelope, state),
            DispatcherEnum::Delete(handler) => handler.dispatch(envelope, state),

            DispatcherEnum::Gain(handler) => handler.dispatch(envelope, state),
            DispatcherEnum::Normalize(handler) => handler.dispatch(envelope, state),
            DispatcherEnum::HighPass(handler) => handler.dispatch(envelope, state),
            DispatcherEnum::LowPass(handler) => handler.dispatch(envelope, state),
            DispatcherEnum::RunScript(handler) => handler.dispatch(envelope, state),

            DispatcherEnum::Play(handler)=>handler.dispatch(envelope, state),
            DispatcherEnum::Pause(handler)=>handler.dispatch(envelope, state),
            DispatcherEnum::Pause(handler)=>handler.dispatch(envelope, state)
        }
    }
}
