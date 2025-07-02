use domain::actors::messages::user::get_user_state::GetUserStateResult;

#[async_trait::async_trait]
pub trait UserOperations{
    async fn get_state(&self)->Result<GetUserStateResult,String>;
}