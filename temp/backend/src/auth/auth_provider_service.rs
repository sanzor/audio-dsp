use std::sync::Arc;

use async_trait::async_trait;
use tracing::{error, info, warn};

use crate::auth::accept_invite_params::AcceptInviteParams;
use crate::auth::accept_invite_result::AcceptInviteResult;
use crate::auth::auth_membership::AuthMembership;
use crate::auth::auth_provider::AuthProvider;
use crate::auth::email_sender::EmailSender;
use crate::auth::invite_user_params::InviteUserParams;
use crate::auth::invite_user_result::InviteUserResult;
use crate::auth::login_params::LoginParams;
use crate::auth::login_result::LoginResult;
use crate::auth::register_user_params::RegisterUserParams;
use crate::auth::register_user_result::RegisterUserResult;
use crate::auth::user::User;
use crate::auth::verify_user_params::VerifyUserParams;
use crate::auth::verify_user_result::VerifyUserResult;
use crate::domain::db::db_organization::OrganizationId;
use crate::domain::db::db_user::UserId;
use crate::domain::service_error::ServiceError;
use crate::membership_roles::membership_roles_provider::MembershipRolesProvider;
use crate::memberships::create_membership_params::CreateMembershipParams;
use crate::memberships::memberships_provider::MembershipsProvider;
use crate::middlewares::jwt::jwt_provider::JwtProvider;
use crate::roles::roles_provider::RolesProvider;
use crate::users::create_user_params::CreateUserParams;
use crate::users::update_user_params::UpdateUserParams;
use crate::users::user_provider::UserProvider;

pub struct AuthProviderService {
    users_provider: Arc<dyn UserProvider>,
    memberships_provider: Arc<dyn MembershipsProvider>,
    roles_provider: Arc<dyn RolesProvider>,
    membership_roles_provider: Arc<dyn MembershipRolesProvider>,
    jwt_provider: Arc<dyn JwtProvider>,
    email_sender: Arc<dyn EmailSender>,
}

impl AuthProviderService {
    pub fn new(
        users_provider: Arc<dyn UserProvider>,
        memberships_provider: Arc<dyn MembershipsProvider>,
        roles_provider: Arc<dyn RolesProvider>,
        membership_roles_provider: Arc<dyn MembershipRolesProvider>,
        jwt_provider: Arc<dyn JwtProvider>,
        email_sender: Arc<dyn EmailSender>,
    ) -> Self {
        Self {
            users_provider,
            memberships_provider,
            roles_provider,
            membership_roles_provider,
            jwt_provider,
            email_sender,
        }
    }

    fn map_user(user: crate::users::user_result::UserResult) -> User {
        User {
            id: user.id,
            email: user.email,
            full_name: user.full_name,
            is_active: user.is_active,
            is_verified: user.is_verified,
        }
    }
}

#[async_trait]
impl AuthProvider for AuthProviderService {
    async fn login(&self, params: LoginParams) -> Result<LoginResult, ServiceError> {
        info!(email = %params.email, "auth login requested");

        let user = self
            .users_provider
            .get_user_by_email(params.email)
            .await?
            .ok_or_else(|| "user not found".to_string())?;

        let token = self.jwt_provider.issue_user_token(user.id)?;

        Ok(LoginResult {
            user: Self::map_user(user),
            token,
            mfa_required: false,
        })
    }

    async fn register(
        &self,
        params: RegisterUserParams,
    ) -> Result<RegisterUserResult, ServiceError> {
        info!(email = %params.email, "auth register requested");

        let created = self
            .users_provider
            .create_user(CreateUserParams {
                email: params.email,
                password_hash: params.password_hash,
                full_name: params.full_name,
                is_active: Some(true),
                is_verified: Some(false),
            })
            .await?;

        let verification_token = self
            .jwt_provider
            .issue_verification_token(created.user.id)?;

        let verification_body = format!(
            "Please verify your email address using this token:\n\n{}\n\nThis token will expire in 24 hours.",
            verification_token
        );
        let email_sent_note = match self
            .email_sender
            .send_email(
                &created.user.email,
                "Verify your email address",
                &verification_body,
            )
            .await
        {
            Ok(_) => None,
            Err(e) => {
                let note = "Failed to send verification email during registration";
                // Log the error so you can see it in your terminal/logs
                error!(error = ?e, note);
                Some(note.to_string())
            }
        };

        let token = self.jwt_provider.issue_user_token(created.user.id)?;

        Ok(RegisterUserResult {
            user: Self::map_user(created.user),
            token,
            mfa_required: false,
            mfa_secret: None,
            email_sent_note,
        })
    }
    async fn verify(&self, params: VerifyUserParams) -> Result<VerifyUserResult, ServiceError> {
        info!(user_id = %params.user_id, "auth verify requested");

        let updated = self
            .users_provider
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
            .ok_or_else(|| "user not found".to_string())?;

        Ok(VerifyUserResult {
            user: Self::map_user(updated.user),
        })
    }

    async fn resend_verification(&self, email: String) -> Result<(), ServiceError> {
        info!(email = %email, "resend verification requested");

        let user = self
            .users_provider
            .get_user_by_email(email)
            .await?
            .ok_or_else(|| "user not found".to_string())?;

        if user.is_verified {
            return Err(ServiceError::Conflict(
                "User is already verified".to_string(),
            ));
        }

        let verification_token = self.jwt_provider.issue_verification_token(user.id)?;

        let resend_body = format!(
            "Please verify your email address using this token:\n\n{}\n\nThis token will expire in 24 hours.",
            verification_token
        );
        self.email_sender
            .send_email(&user.email, "Verify your email address", &resend_body)
            .await?;

        Ok(())
    }

    async fn accept_invite(
        &self,
        params: AcceptInviteParams,
    ) -> Result<AcceptInviteResult, ServiceError> {
        info!(
            user_id = params.user_id,
            org_id = params.org_id,
            "auth accept-invite requested"
        );

        let user_id =
            UserId::try_from(params.user_id).map_err(|_| "user id out of range".to_string())?;
        let org_id = OrganizationId::try_from(params.org_id)
            .map_err(|_| "org id out of range".to_string())?;

        let existing = self
            .memberships_provider
            .get_membership(user_id, org_id)
            .await?;
        if existing.is_none() {
            self.memberships_provider
                .create_membership(CreateMembershipParams { user_id, org_id })
                .await?;
        }

        let membership_roles = self
            .membership_roles_provider
            .list_membership_roles(Some(user_id), Some(org_id), None)
            .await?;

        let mut roles = Vec::new();
        for mr in membership_roles {
            match self.roles_provider.get_role(mr.role_id).await {
                Ok(Some(role)) => roles.push(role.name),
                Ok(None) => warn!(
                    user_id,
                    org_id,
                    role_id = mr.role_id,
                    "role not found for membership role"
                ),
                Err(e) => warn!(
                    user_id,
                    org_id,
                    role_id = mr.role_id,
                    error = %e,
                    "role lookup failed"
                ),
            }
        }

        let token = self.jwt_provider.issue_org_token(user_id, org_id)?;

        Ok(AcceptInviteResult {
            membership: AuthMembership {
                user_id,
                org_id,
                token,
                roles,
            },
        })
    }

    async fn invite_user(
        &self,
        params: InviteUserParams,
    ) -> Result<InviteUserResult, ServiceError> {
        info!(email = %params.email, org_id = params.org_id, "auth invite-user requested");

        let user = self
            .users_provider
            .get_user_by_email(params.email)
            .await?
            .ok_or(ServiceError::NotFound)?;

        let invite_token = self.jwt_provider.issue_verification_token(user.id)?;

        let invite_body = format!(
            "You have been invited to join an organization.\n\nUse this token to accept the invite:\n\n{}\n\nThis token will expire in 24 hours.",
            invite_token
        );
        self.email_sender
            .send_email(&user.email, "You've been invited", &invite_body)
            .await?;

        Ok(InviteUserResult { user_id: user.id })
    }
}
