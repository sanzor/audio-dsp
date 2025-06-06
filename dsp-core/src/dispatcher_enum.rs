use std::sync::{Arc, Mutex};

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
use async_trait::async_trait;
use dsp_domain::{dsp_message_result::DspMessageResult, envelope::Envelope};

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
#[async_trait]
impl CommandDispatch for DispatcherEnum {
    async fn dispatch(
        &self,
        envelope: Envelope,
        state: Arc<Mutex<SharedState>>,
    ) -> Result<DspMessageResult, String> {
        match self {
            DispatcherEnum::Load(handler) => handler.dispatch(envelope, state).await,
            DispatcherEnum::Info(handler) => handler.dispatch(envelope, state).await,
            DispatcherEnum::List(handler) => handler.dispatch(envelope, state).await,
            DispatcherEnum::Upload(handler) => handler.dispatch(envelope, state).await,
            DispatcherEnum::Copy(handler) => handler.dispatch(envelope, state).await,
            DispatcherEnum::Delete(handler) => handler.dispatch(envelope, state).await,

            DispatcherEnum::Gain(handler) => handler.dispatch(envelope, state).await,
            DispatcherEnum::Normalize(handler) => handler.dispatch(envelope, state).await,
            DispatcherEnum::HighPass(handler) => handler.dispatch(envelope, state).await,
            DispatcherEnum::LowPass(handler) => handler.dispatch(envelope, state).await,
            DispatcherEnum::RunScript(handler) => handler.dispatch(envelope, state).await,

            DispatcherEnum::Play(handler) => handler.dispatch(envelope, state).await,
            DispatcherEnum::Pause(handler) => handler.dispatch(envelope, state).await,
            DispatcherEnum::Stop(handler) => handler.dispatch(envelope, state).await,
        }
    }
}
