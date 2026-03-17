use std::sync::Arc;

use async_trait::async_trait;
use tracing::info;

use crate::domain::db::db_user::UserId;
use crate::domain::service_error::ServiceError;
use crate::users::{
    create_user_params::CreateUserParams, create_user_result::CreateUserResult,
    data_provider::user_data_provider::UserDataProvider, update_user_params::UpdateUserParams,
    update_user_result::UpdateUserResult, user_crud_provider::UserCrudProvider,
    user_result::UserResult,
};

pub struct UserCrudProviderService {
    data_provider: Arc<dyn UserDataProvider>,
}

impl UserCrudProviderService {
    pub fn new(data_provider: Arc<dyn UserDataProvider>) -> Self {
        Self { data_provider }
    }
}

#[async_trait]
impl UserCrudProvider for UserCrudProviderService {
    async fn create_user(
        &self,
        params: CreateUserParams,
    ) -> Result<CreateUserResult, ServiceError> {
        info!(email = %params.email, "create user requested");
        let record = self.data_provider.create_user(params).await?;
        Ok(CreateUserResult { user: UserResult::from(record) })
    }

    async fn update_user(
        &self,
        user_id: UserId,
        params: UpdateUserParams,
    ) -> Result<Option<UpdateUserResult>, ServiceError> {
        info!(user_id, "update user requested");
        let record = self.data_provider.update_user(user_id, params).await?;
        Ok(record.map(|user| UpdateUserResult { user: UserResult::from(user) }))
    }

    async fn delete_user(&self, user_id: UserId) -> Result<bool, ServiceError> {
        info!(user_id, "delete user requested");
        Ok(self.data_provider.delete_user(user_id).await?)
    }

    async fn get_user(&self, user_id: UserId) -> Result<Option<UserResult>, ServiceError> {
        info!(user_id, "get user requested");
        let record = self.data_provider.get_user(user_id).await?;
        Ok(record.map(UserResult::from))
    }

    async fn get_user_by_email(&self, email: String) -> Result<Option<UserResult>, ServiceError> {
        info!(email = %email, "get user by email requested");
        let record = self.data_provider.get_user_by_email(email).await?;
        Ok(record.map(UserResult::from))
    }

    async fn get_all_users(&self) -> Result<Vec<UserResult>, ServiceError> {
        info!("get all users requested");
        let records = self.data_provider.get_all_users().await?;
        Ok(records.into_iter().map(UserResult::from).collect())
    }
}
