use std:: sync::Arc;
use dsp_core::{command_processor::CommandProcessor, state::SharedState};
use tokio::sync::Mutex;

use actix::{Actor, Context, Handler, ResponseFuture};

use dsp_domain::{dsp_message::DspMessage, dsp_message_result::DspMessageResult};

struct UserActor {
    processor: Arc<CommandProcessor>,
    state: Arc<Mutex<SharedState>>,
}

impl Actor for UserActor {
    type Context = Context<Self>;
}

impl Handler<DspMessage> for UserActor {
    type Result = ResponseFuture<Result<DspMessageResult, String>>;

    // async move variant
    fn handle(&mut self, msg: DspMessage, ctx: &mut Self::Context) -> Self::Result {
        let proc = Arc::clone(&self.processor);

        // Lock the Mutex to access the mutable state
        let state_guard = Arc::clone(&self.state);
        Box::pin(async move { proc.process_command(msg, state_guard).await })
    }
}
