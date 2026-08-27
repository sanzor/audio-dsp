use async_trait::async_trait;
use domain::domain_user::UserId;


use crate::domain::service_error::ServiceError;
use crate::users::{
    create_user_params::CreateUserParams, create_user_result::CreateUserResult,
    update_user_params::UpdateUserParams, update_user_result::UpdateUserResult,
    user_result::UserResult,
};

#[async_trait]
pub trait UserProvider: Send + Sync {
    async fn create_user(
        &self,
        params: CreateUserParams,
    ) -> Result<CreateUserResult, ServiceError>;
    async fn update_user(
        &self,
        user_id: UserId,
        params: UpdateUserParams,
    ) -> Result<Option<UpdateUserResult>, ServiceError>;
    async fn delete_user(&self, user_id: UserId) -> Result<bool, ServiceError>;
    async fn get_user(&self, user_id: UserId) -> Result<Option<UserResult>, ServiceError>;
    async fn get_user_by_email(&self, email: &str) -> Result<Option<UserResult>, ServiceError>;
    async fn get_all_users(&self) -> Result<Vec<UserResult>, ServiceError>;
}
