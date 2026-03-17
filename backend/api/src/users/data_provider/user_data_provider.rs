use async_trait::async_trait;

use crate::{
    domain::{
        data_error::DataError,
        db::db_user::{DbUser, UserId},
    },
    users::{create_user_params::CreateUserParams, update_user_params::UpdateUserParams},
};

#[async_trait]
pub trait UserDataProvider: Send + Sync {
    async fn create_user(&self, params: CreateUserParams) -> Result<DbUser, DataError>;
    async fn update_user(
        &self,
        user_id: UserId,
        params: UpdateUserParams,
    ) -> Result<Option<DbUser>, DataError>;
    async fn delete_user(&self, user_id: UserId) -> Result<bool, DataError>;
    async fn get_user(&self, user_id: UserId) -> Result<Option<DbUser>, DataError>;
    async fn get_user_by_email(&self, email: String) -> Result<Option<DbUser>, DataError>;
    async fn get_all_users(&self) -> Result<Vec<DbUser>, DataError>;
}
