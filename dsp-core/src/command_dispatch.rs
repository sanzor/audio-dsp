use async_trait::async_trait;
use dsp_domain::{envelope::Envelope, tracks_message_result::TracksMessageResult};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::state::SharedState;

#[async_trait]
pub(crate) trait CommandDispatch {
    async fn dispatch(
        &self,
        envelope: Envelope,
        state: Arc<Mutex<SharedState>>,
    ) -> Result<TracksMessageResult, String>;
}
