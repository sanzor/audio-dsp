use domain::actors::messages::player::get_player_state::{GetPlayerState, GetPlayerStateResult};
use kameo::prelude::{Context, Message};

use crate::audio_player_actor::audio_player_actor::AudioPlayerActor;

impl Message<GetPlayerState> for AudioPlayerActor {
    type Reply = Result<GetPlayerStateResult, String>;

    async fn handle(
        &mut self,
        msg: GetPlayerState,
        ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        Ok((GetPlayerStateResult {
            cursor: self.cursor,
            state: self.state.clone(),
            written: self.frames_written,
        }))
    }
}
