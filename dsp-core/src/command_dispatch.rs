use async_trait::async_trait;
use domain::{envelope::Envelope, tracks_message_result::TracksMessageResult};

use crate::state::TracksState;

#[async_trait]
pub(crate) trait CommandDispatch {
    async fn dispatch(
        &self,
        envelope: Envelope,
        state: &mut TracksState,
    ) -> Result<TracksMessageResult, String>;
}
