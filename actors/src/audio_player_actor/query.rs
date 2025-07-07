use domain::actors::messages::{
    player::get_player_state::PlayerStateResult, user::get_user_state::PlayerStateQueryResult,
};

use crate::audio_player_actor::audio_player_actor::AudioPlayerActor;

impl Message<PlayerStateQuery> for AudioPlayerActor {
    type Reply = Result<PlayerStateQueryResult, String>;

    async fn handle(
        &mut self,
        msg: PlayerStateQuery,
        ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        Ok((PlayerStateResult {
            cursor: self.cursor,
            state: self.state.clone(),
            written: self.frames_written,
        }))
    }
}
