use std::sync::Arc;
use tracing::{error, info, warn};

use domain::workspace_role::WorkspaceRole;

use crate::{
    memberships::memberships_provider::{CreateMembershipParams, MembershipsProvider},
    users::{
        create_user_params::CreateUserParams, update_user_params::UpdateUserParams,
        user_provider::UserProvider, user_result::UserResult,
    },
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

    fn map_user(u: UserResult) -> AuthUser {
        AuthUser {
            id: u.id,
            email: u.email,
            name: u.full_name,
            is_admin: false,
            is_active: u.is_active,
            is_verified: u.is_verified,
        }
    }
}

#[async_trait::async_trait]
impl AuthProvider for AuthProviderService {
    async fn login(&self, params: LoginParams) -> Result<LoginResult, ServiceError> {
        info!(email = %params.email, "login requested");

        let user = self
            .user_provider
            .get_user_by_email(&params.email)
            .await?;
        let user = user.ok_or(ServiceError::NotFound)?;

        
        let token = self.jwt_provider.issue_user_token(
            user.id,
            Some(&user.full_name),
            Some(&user.email),
            false,
        )?;

        Ok(LoginResult { user: Self::map_user(user), token })
    }

    async fn register(&self, params: RegisterUserParams) -> Result<RegisterUserResult, ServiceError> {
        info!(email = %params.email, "register requested");

        let created = self
            .user_provider
            .create_user(CreateUserParams {
                email: params.email,
                password_hash: params.password_hash,
                full_name: params.name,
                is_active: Some(true),
                is_verified: Some(false),
            })
            .await?;

        let verification_token = self.jwt_provider.issue_verification_token(created.user.id)?;
        let body = format!(
            "Verify your email with this token:\n\n{verification_token}\n\nExpires in 24 hours."
        );
        let email_sent_note = match self
            .email_sender
            .send_email(&created.user.email, "Verify your email", &body)
            .await
        {
            Ok(_) => None,
            Err(e) => {
                error!(error = %e, "failed to send verification email");
                Some("Verification email could not be sent".to_string())
            }
        };

        let token = self.jwt_provider.issue_user_token(
            created.user.id,
            Some(&created.user.full_name),
            Some(&created.user.email),
            false,
        )?;
        Ok(RegisterUserResult {
            user: Self::map_user(created.user),
            token,
            email_sent_note,
        })
    }

    async fn verify(&self, params: VerifyUserParams) -> Result<VerifyUserResult, ServiceError> {
        info!(user_id = %params.user_id, "verify requested");

        let updated = self
            .user_provider
            .update_user(
                params.user_id,
                UpdateUserParams {
                    email: None,
                    password_hash: None,
                    full_name: None,
                    is_active: None,
                    is_verified: Some(true),
                },
            )
            .await?
            .ok_or(ServiceError::NotFound)?;

        Ok(VerifyUserResult {
            user: Self::map_user(updated.user),
        })
    }

    async fn resend_verification(&self, email: String) -> Result<(), ServiceError> {
        info!(email = %email, "resend verification requested");

        let user = self.user_provider.get_user_by_email(&email).await?
            .ok_or(ServiceError::NotFound)?;

        if user.is_verified {
            return Err(ServiceError::Conflict("user already verified".to_string()));
        }

        let token = self.jwt_provider.issue_verification_token(user.id)?;
        let body = format!("Verify your email:\n\n{token}\n\nExpires in 24 hours.");
        self.email_sender.send_email(&user.email, "Verify your email", &body).await?;

        Ok(())
    }

    async fn invite_user(&self, params: InviteUserParams) -> Result<InviteUserResult, ServiceError> {
        info!(email = %params.email, workspace_id = %params.workspace_id, role = %params.role, "invite requested");

        // Issue token with invitee email — user does not need to exist yet
        let token = self.jwt_provider.issue_invite_token(&params.email, params.workspace_id, &params.role.to_string())?;

        let frontend_url = std::env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:5173".to_string());
        let accept_link = format!("{frontend_url}/accept-invite?token={token}");
        let body = format!(
            "You've been invited to a workspace.\n\nAccept your invitation here:\n\n{accept_link}\n\nExpires in 7 days."
        );
        if let Err(e) = self.email_sender.send_email(&params.email, "Workspace invitation", &body).await {
            error!(error = %e, "failed to send invite email");
        }

        Ok(InviteUserResult { invitee_email: params.email })
    }

    async fn accept_invite(&self, params: AcceptInviteParams) -> Result<AcceptInviteResult, ServiceError> {
        info!(caller_user_id = %params.caller_user_id, "accept-invite requested");

        let claims = self.jwt_provider.verify(&params.invite_token)?;

        if claims.purpose.as_deref() != Some("invite") {
            return Err(ServiceError::Internal("invalid token purpose".to_string()));
        }

        let invitee_email = claims.email
            .ok_or_else(|| ServiceError::Internal("missing email in invite token".to_string()))?;
        let workspace_id = claims.workspace_id
            .ok_or_else(|| ServiceError::Internal("missing workspace_id in token".to_string()))?;
        let role_str = claims.role
            .ok_or_else(|| ServiceError::Internal("missing role in token".to_string()))?;
        let role = WorkspaceRole::from_str(&role_str)
            .ok_or_else(|| ServiceError::Internal(format!("unknown role: {role_str}")))?;

        // Verify that the authenticated caller's email matches the invite
        let caller = self
            .user_provider
            .get_user(params.caller_user_id)
            .await?
            .ok_or(ServiceError::NotFound)?;
        if caller.email.to_lowercase() != invitee_email.to_lowercase() {
            warn!(caller_email = %caller.email, invitee_email = %invitee_email, "email mismatch on accept-invite");
            return Err(ServiceError::Forbidden);
        }

        self.memberships_provider.create_membership(CreateMembershipParams {
            user_id: caller.id,
            workspace_id,
            role: role.clone(),
        }).await?;

        Ok(AcceptInviteResult {
            user_id: caller.id,
            workspace_id,
            role,
        })
    }
}
