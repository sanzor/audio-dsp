use async_trait::async_trait;
use dsp_domain::{dsp_command_result::DspCommandResult, envelope::Envelope};

use crate::state::{SharedState, State};

#[async_trait]
pub(crate) trait CommandDispatch {
    async fn dispatch(
        &self,
        envelope: Envelope,
        state: SharedState,
    ) -> Result<DspCommandResult, String>;
}
