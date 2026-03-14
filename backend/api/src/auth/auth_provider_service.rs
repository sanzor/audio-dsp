use std::sync::Arc;
use tracing::{error, info, warn};

use domain::{create_domain_user_params::CreateDomainUserParams, project_role::ProjectRole};

use crate::{
    memberships::memberships_provider::{CreateMembershipParams, MembershipsProvider},
    users::user_provider::UserProvider,
};

use super::{
    accept_invite_params::AcceptInviteParams,
    accept_invite_result::AcceptInviteResult,
    auth_provider::AuthProvider,
    email_sender::EmailSender,
    invite_user_params::InviteUserParams,
    invite_user_result::InviteUserResult,
    jwt_provider::JwtProvider,
    login_params::LoginParams,
    login_result::LoginResult,
    register_user_params::RegisterUserParams,
    register_user_result::RegisterUserResult,
    service_error::ServiceError,
    user::AuthUser,
    verify_user_params::VerifyUserParams,
    verify_user_result::VerifyUserResult,
};

pub struct AuthProviderService {
    user_provider: Arc<dyn UserProvider>,
    memberships_provider: Arc<dyn MembershipsProvider>,
    jwt_provider: Arc<dyn JwtProvider>,
    email_sender: Arc<dyn EmailSender>,
}

impl AuthProviderService {
    pub fn new(
        user_provider: Arc<dyn UserProvider>,
        memberships_provider: Arc<dyn MembershipsProvider>,
        jwt_provider: Arc<dyn JwtProvider>,
        email_sender: Arc<dyn EmailSender>,
    ) -> Self {
        Self { user_provider, memberships_provider, jwt_provider, email_sender }
    }

    fn map_user(u: &domain::domain_user::DomainUser) -> AuthUser {
        AuthUser {
            id: u.id.clone(),
            email: u.email.clone(),
            name: u.name.clone(),
            is_active: u.is_active,
            is_verified: u.is_verified,
        }
    }
}

#[async_trait::async_trait]
impl AuthProvider for AuthProviderService {
    async fn login(&self, params: LoginParams) -> Result<LoginResult, ServiceError> {
        info!(email = %params.email, "login requested");

        let user = self.user_provider.get_user_by_email(&params.email).await
            .ok_or(ServiceError::NotFound)?;

        let stored = user.password_hash.as_deref().unwrap_or("");
        if stored != params.password_hash {
            return Err(ServiceError::NotFound);
        }

        let token = self.jwt_provider.issue_user_token(
            &user.id,
            Some(&user.name),
            Some(&user.email),
            user.is_admin,
        )?;

        Ok(LoginResult { user: Self::map_user(&user), token })
    }

    async fn register(&self, params: RegisterUserParams) -> Result<RegisterUserResult, ServiceError> {
        info!(email = %params.email, "register requested");

        let user = self.user_provider.create_domain_user(CreateDomainUserParams {
            email: params.email,
            name: params.name,
            picture: String::new(),
            google_sub_id: None,
            password_hash: Some(params.password_hash),
        }).await?;

        let verification_token = self.jwt_provider.issue_verification_token(&user.id)?;
        let body = format!(
            "Verify your email with this token:\n\n{verification_token}\n\nExpires in 24 hours."
        );
        let email_sent_note = match self.email_sender.send_email(&user.email, "Verify your email", &body).await {
            Ok(_) => None,
            Err(e) => {
                error!(error = %e, "failed to send verification email");
                Some("Verification email could not be sent".to_string())
            }
        };

        let token = self.jwt_provider.issue_user_token(&user.id, Some(&user.name), Some(&user.email), user.is_admin)?;
        Ok(RegisterUserResult { user: Self::map_user(&user), token, email_sent_note })
    }

    async fn verify(&self, params: VerifyUserParams) -> Result<VerifyUserResult, ServiceError> {
        info!(user_id = %params.user_id, "verify requested");

        let mut user = self.user_provider.get_user_by_id(&params.user_id).await
            .ok_or(ServiceError::NotFound)?;

        user.is_verified = true;
        self.user_provider.update_user(user.clone()).await?;

        Ok(VerifyUserResult { user: Self::map_user(&user) })
    }

    async fn resend_verification(&self, email: String) -> Result<(), ServiceError> {
        info!(email = %email, "resend verification requested");

        let user = self.user_provider.get_user_by_email(&email).await
            .ok_or(ServiceError::NotFound)?;

        if user.is_verified {
            return Err(ServiceError::Conflict("user already verified".to_string()));
        }

        let token = self.jwt_provider.issue_verification_token(&user.id)?;
        let body = format!("Verify your email:\n\n{token}\n\nExpires in 24 hours.");
        self.email_sender.send_email(&user.email, "Verify your email", &body).await?;

        Ok(())
    }

    async fn invite_user(&self, params: InviteUserParams) -> Result<InviteUserResult, ServiceError> {
        info!(email = %params.email, project_id = %params.project_id, role = %params.role, "invite requested");

        let user = self.user_provider.get_user_by_email(&params.email).await
            .ok_or(ServiceError::NotFound)?;

        let token = self.jwt_provider.issue_invite_token(&user.id, &params.project_id, &params.role.to_string())?;
        let body = format!(
            "You've been invited to a project.\n\nUse this token to accept:\n\n{token}\n\nExpires in 7 days."
        );
        self.email_sender.send_email(&user.email, "Project invitation", &body).await?;

        Ok(InviteUserResult { user_id: user.id })
    }

    async fn accept_invite(&self, params: AcceptInviteParams) -> Result<AcceptInviteResult, ServiceError> {
        info!("accept-invite requested");

        let claims = self.jwt_provider.verify(&params.invite_token)?;

        if claims.purpose.as_deref() != Some("invite") {
            return Err(ServiceError::Internal("invalid token purpose".to_string()));
        }

        let project_id = claims.project_id
            .ok_or_else(|| ServiceError::Internal("missing project_id in token".to_string()))?;
        let role_str = claims.role
            .ok_or_else(|| ServiceError::Internal("missing role in token".to_string()))?;
        let role = ProjectRole::from_str(&role_str)
            .ok_or_else(|| ServiceError::Internal(format!("unknown role: {role_str}")))?;

        self.memberships_provider.create_membership(CreateMembershipParams {
            user_id: claims.user_id.clone(),
            project_id: project_id.clone(),
            role: role.clone(),
        }).await?;
        
        Ok(AcceptInviteResult {
            user_id: claims.user_id,
            project_id,
            role,
        })
    }
}
