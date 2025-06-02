use async_trait::async_trait;
use dsp_domain::{dsp_message_result::DspMessageResult, envelope::Envelope};

use crate::state::{SharedState, State};

#[async_trait]
pub(crate) trait CommandDispatch {
    async fn dispatch(
        &self,
        envelope: Envelope,
        state: SharedState,
    ) -> Result<DspMessageResult, String>;
}
