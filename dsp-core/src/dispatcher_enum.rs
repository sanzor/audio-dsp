use async_trait::async_trait;
use dsp_domain::{dsp_message_result::DspMessageResult, envelope::Envelope};

use crate::actors::user_actor::user_actor_state::UserActorState;
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
#[async_trait]
impl CommandDispatch for DispatcherEnum {
    async fn dispatch_mut(
        &self,
        envelope: Envelope,
        state:&mut SharedState,
    ) -> Result<DspMessageResult, String> {
        match self {
            DispatcherEnum::Load(handler) => handler.dispatch_mut(envelope, state).await,
            DispatcherEnum::Info(handler) => handler.dispatch_mut(envelope, state).await,
            DispatcherEnum::List(handler) => handler.dispatch_mut(envelope, state).await,
            DispatcherEnum::Upload(handler) => handler.dispatch_mut(envelope, state).await,
            DispatcherEnum::Copy(handler) => handler.dispatch_mut(envelope, state).await,
            DispatcherEnum::Delete(handler) => handler.dispatch_mut(envelope, state).await,

            DispatcherEnum::Gain(handler) => handler.dispatch_mut(envelope, state).await,
            DispatcherEnum::Normalize(handler) => handler.dispatch_mut(envelope, state).await,
            DispatcherEnum::HighPass(handler) => handler.dispatch_mut(envelope, state).await,
            DispatcherEnum::LowPass(handler) => handler.dispatch_mut(envelope, state).await,
            DispatcherEnum::RunScript(handler) => handler.dispatch_mut(envelope, state).await,

            DispatcherEnum::Play(handler) => handler.dispatch_mut(envelope, state).await,
            DispatcherEnum::Pause(handler) => handler.dispatch_mut(envelope, state).await,
            DispatcherEnum::Stop(handler) => handler.dispatch_mut(envelope, state).await,
        }
    }
}
