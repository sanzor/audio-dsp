use crate::command_dispatch::CommandDispatch;
use crate::crud_dispatchers::{
    CopyDispatcher, DeleteDispatcher, InfoDispatcher, InsertDispatcher, ListDispatcher,
    LoadDispatcher, RunScriptDispatcher, UploadDispatcher,
};
use crate::dsp_dispatchers::{
    GainDispatcher, HighPassDispatcher, LowPassDispatcher, NormalizeDispatcher,
};
use crate::state::SharedState;
use async_trait::async_trait;
use dsp_domain::{envelope::Envelope, tracks_message_result::TracksMessageResult};
use std::sync::Arc;
use tokio::sync::Mutex;

pub enum DispatcherEnum {
    Load(LoadDispatcher),
    Insert(InsertDispatcher),
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
}
#[async_trait]
impl CommandDispatch for DispatcherEnum {
    async fn dispatch(
        &self,
        envelope: Envelope,
        state: Arc<Mutex<SharedState>>,
    ) -> Result<TracksMessageResult, String> {
        match self {
            DispatcherEnum::Load(handler) => handler.dispatch(envelope, state).await,
            DispatcherEnum::Insert(handler) => handler.dispatch(envelope, state).await,
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
        }
    }
}
