use async_trait::async_trait;
use domain::{
    actors::{user_crud_command::UserCrudCommand, user_crud_command_result::UserCrudCommandResult}, track::Track
};


use crate::{crud_dispatchers::crud_command_dispatch::CrudCommandDispatch, state::TracksState};

pub struct InsertTrackDispatcher {}
#[async_trait]
impl CrudCommandDispatch for InsertTrackDispatcher {
    async fn dispatch(
        &self,
        command: UserCrudCommand,
        state: &mut TracksState,
    ) -> Result<UserCrudCommandResult, String> {
        match command {
            UserCrudCommand::InsertTrack {
               track
            } => {
                self.internal_dispatch(track, state)
                    .await
            }
            _ => Err("".to_owned()),
        }
    }
}

impl InsertTrackDispatcher {
    async fn internal_dispatch(
        &self,
        track: Track,
        state: &mut TracksState,
    ) -> Result<UserCrudCommandResult, String> {

        state.upsert_track(track).await?;

        Ok(UserCrudCommandResult {
            output: format!("Inserted track")
        })
    }
}
