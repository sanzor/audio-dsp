use async_trait::async_trait;
use dsp_domain::{dsp_message_result::DspMessageResult, envelope::Envelope};

use crate::state::SharedState;

#[async_trait]
pub(crate) trait CommandDispatch {
    async fn dispatch_mut(
        &self,
        envelope: Envelope,
        state: & mut SharedState,
    ) -> Result<DspMessageResult, String>;
}
