use async_trait::async_trait;

use crate::auth::invite_user_params::InviteUserParams;
use crate::auth::invite_user_result::InviteUserResult;
use crate::auth::{
    accept_invite_params::AcceptInviteParams, accept_invite_result::AcceptInviteResult,
    login_params::LoginParams, login_result::LoginResult, register_user_params::RegisterUserParams,
    register_user_result::RegisterUserResult, verify_user_params::VerifyUserParams,
    verify_user_result::VerifyUserResult,
};
use crate::domain::service_error::ServiceError;

#[async_trait]
pub trait AuthProvider {
    async fn login(&self, params: LoginParams) -> Result<LoginResult, ServiceError>;
    async fn register(
        &self,
        params: RegisterUserParams,
    ) -> Result<RegisterUserResult, ServiceError>;
    async fn verify(&self, params: VerifyUserParams) -> Result<VerifyUserResult, ServiceError>;
    async fn resend_verification(&self, email: String) -> Result<(), ServiceError>;
    async fn accept_invite(
        &self,
        params: AcceptInviteParams,
    ) -> Result<AcceptInviteResult, ServiceError>;
    async fn invite_user(&self, params: InviteUserParams)
        -> Result<InviteUserResult, ServiceError>;
}
