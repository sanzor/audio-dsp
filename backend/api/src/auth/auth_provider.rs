use super::{
    accept_invite_params::AcceptInviteParams, accept_invite_result::AcceptInviteResult,
    invite_user_params::InviteUserParams, invite_user_result::InviteUserResult,
    login_params::LoginParams, login_result::LoginResult, register_user_params::RegisterUserParams,
    register_user_result::RegisterUserResult, service_error::ServiceError,
    verify_user_params::VerifyUserParams, verify_user_result::VerifyUserResult,
};

#[async_trait::async_trait]
pub trait AuthProvider: Send + Sync {
    async fn login(&self, params: LoginParams) -> Result<LoginResult, ServiceError>;
    async fn register(
        &self,
        params: RegisterUserParams,
    ) -> Result<RegisterUserResult, ServiceError>;
    async fn verify(&self, params: VerifyUserParams) -> Result<VerifyUserResult, ServiceError>;
    async fn resend_verification(&self, email: String) -> Result<(), ServiceError>;
    async fn invite_user(&self, params: InviteUserParams)
        -> Result<InviteUserResult, ServiceError>;
    async fn accept_invite(
        &self,
        params: AcceptInviteParams,
    ) -> Result<AcceptInviteResult, ServiceError>;
}
