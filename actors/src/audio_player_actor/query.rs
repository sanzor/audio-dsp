use crate::audio_player_actor::audio_player_actor::AudioPlayerActor;

impl Message<PlayerStateQuery> for AudioPlayerActor {
    type Reply = Result<PlayerStateQueryResult, String>;

    async fn handle(
        &mut self,
        msg: PlayerStateQuery,
        ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        Ok((PlayerStateQueryResult {
            cursor: self.cursor,
            state: self.state.clone(),
            written: self.frames_written,
        }))
    }
}