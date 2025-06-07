use async_trait::async_trait;
use dsp_domain::{dsp_message_result::DspMessageResult, envelope::Envelope};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::state::SharedState;

#[async_trait]
pub(crate) trait CommandDispatch {
    async fn dispatch(
        &self,
        envelope: Envelope,
        state: Arc<Mutex<SharedState>>,
    ) -> Result<DspMessageResult, String>;
}
