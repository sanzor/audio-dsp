use async_trait::async_trait;
use dsp_domain::{message_result::MessageResult, envelope::Envelope};

use crate::state::{SharedState, State};

#[async_trait]
pub(crate) trait CommandDispatch {
    async fn dispatch(
        &self,
        envelope: Envelope,
        state: SharedState,
    ) -> Result<MessageResult, String>;
}
