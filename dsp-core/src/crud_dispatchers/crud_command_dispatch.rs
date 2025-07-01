use async_trait::async_trait;
use domain::actors::{user_crud_command::UserCrudCommand, user_crud_command_result::UserCrudCommandResult};

use crate::state::TracksState;

#[async_trait]
pub(crate) trait CrudCommandDispatch {
    async fn dispatch(
        &self,
        command: UserCrudCommand,
        state: &mut TracksState,
    ) -> Result<UserCrudCommandResult, String>;
}
