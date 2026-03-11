use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use tracing::{error, info, warn};
use utoipa::ToSchema;

use crate::auth::accept_invite_params::AcceptInviteParams;
use crate::auth::auth_app_data::AuthAppData;
use crate::auth::invite_user_params::InviteUserParams;
use crate::auth::login_params::LoginParams;
use crate::auth::register_user_params::RegisterUserParams;
use crate::auth::user::User;
use crate::auth::verify_user_params::VerifyUserParams;
use crate::domain::db::db_organization::OrganizationId;
use crate::domain::db::db_user::UserId;
use crate::domain::service_error::ServiceError;
use crate::middlewares::jwt::portal_jwt_claims::TokenPurpose;

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct LoginInput {
    pub email: String,
    pub password_hash: String,
    pub mfa_code: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct LoginResult {
    pub user: User,
    pub token: String,
    pub mfa_required: bool,
}

#[utoipa::path(
    post,
    path = "/auth/login",
    tag = "Auth",
    request_body = LoginInput,
    responses(
        (status = 200, description = "Login completed", body = LoginResult),
        (status = 400, description = "Invalid input parameters"),
        (status = 404, description = "Could not find user"),
        (status = 500, description = "Internal server error"),
    )
)]
#[post("/login")]
pub async fn login(
    payload: web::Json<LoginInput>,
    app_state: web::Data<AuthAppData>,
) -> HttpResponse {
    info!("auth login request received");
    let input = payload.into_inner();

    if input.email.trim().is_empty() {
        warn!("login rejected: missing email");
        return HttpResponse::BadRequest().body("email is required");
    }
    if input.password_hash.trim().is_empty() {
        warn!("login rejected: missing password_hash");
        return HttpResponse::BadRequest().body("password_hash is required");
    }

    let params = LoginParams {
        email: input.email,
        password_hash: input.password_hash,
        mfa_code: input.mfa_code,
    };

    match app_state.auth_provider.login(params).await {
        Ok(result) => HttpResponse::Ok().json(LoginResult {
            user: result.user,
            token: result.token,
            mfa_required: result.mfa_required,
        }),
        Err(ServiceError::NotFound) => HttpResponse::NotFound().body("Could not find user"),
        Err(e) => {
            error!(error = %e, "login failed");
            HttpResponse::InternalServerError().body("failed to login")
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct RegisterUserOnlyInput {
    pub email: String,
    pub password_hash: String,
    pub full_name: String,
    pub enable_mfa: Option<bool>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct RegisterUserOnlyResult {
    pub user: User,
    pub token: String,
    pub mfa_required: bool,
    pub mfa_secret: Option<String>,
}

#[utoipa::path(
    post,
    path = "/auth/register-user",
    tag = "Auth",
    request_body = RegisterUserOnlyInput,
    responses(
        (status = 201, description = "User created", body = RegisterUserOnlyResult),
        (status = 400, description = "Invalid input parameters"),
        (status = 500, description = "Internal server error"),
    )
)]
#[post("/register-user")]
pub async fn register_user(
    payload: web::Json<RegisterUserOnlyInput>,
    app_state: web::Data<AuthAppData>,
) -> HttpResponse {
    info!("auth register-user request received");
    let input = payload.into_inner();

    if input.email.trim().is_empty() {
        warn!("register-user rejected: missing email");
        return HttpResponse::BadRequest().body("email is required");
    }
    if input.password_hash.trim().is_empty() {
        warn!("register-user rejected: missing password_hash");
        return HttpResponse::BadRequest().body("password_hash is required");
    }
    if input.full_name.trim().is_empty() {
        warn!("register-user rejected: missing full_name");
        return HttpResponse::BadRequest().body("full_name is required");
    }

    let params = RegisterUserParams {
        email: input.email,
        password_hash: input.password_hash,
        full_name: input.full_name,
    };

    match app_state.auth_provider.register(params).await {
        Ok(result) => HttpResponse::Created().json(RegisterUserOnlyResult {
            user: result.user,
            token: result.token,
            mfa_required: result.mfa_required,
            mfa_secret: result.mfa_secret,
        }),
        Err(e) => {
            error!(error = %e, "register-user failed");
            HttpResponse::InternalServerError().body("failed to register user")
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct VerifyInput {
    pub token: String,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct VerifyOutputResult {
    pub user: User,
}

#[utoipa::path(
    post,
    path = "/auth/verify",
    tag = "Auth",
    request_body = VerifyInput,
    responses(
        (status = 200, description = "Verify succeeded", body = VerifyOutputResult),
        (status = 400, description = "Invalid input parameters"),
        (status = 500, description = "Internal server error"),
    )
)]
#[post("/verify")]
pub async fn verify(
    payload: web::Json<VerifyInput>,
    app_state: web::Data<AuthAppData>,
) -> HttpResponse {
    info!("verify user request received");
    let input = payload.into_inner();

    if input.token.trim().is_empty() {
        warn!("verify rejected: missing token");
        return HttpResponse::BadRequest().body("token is required");
    }

    let claims = match app_state.jwt_provider.verify(&input.token) {
        Ok(c) => c,
        Err(e) => {
            warn!(error = %e, "verify rejected: invalid token");
            return HttpResponse::BadRequest().body("invalid or expired token");
        }
    };

    if claims.purpose != TokenPurpose::Verification {
        warn!("verify rejected: token is not a verification token");
        return HttpResponse::BadRequest().body("invalid token purpose");
    }

    match app_state
        .auth_provider
        .verify(VerifyUserParams {
            user_id: claims.user_id,
        })
        .await
    {
        Ok(result) => HttpResponse::Ok().json(VerifyOutputResult { user: result.user }),
        Err(e) => {
            error!(error = %e, "verify user failed");
            HttpResponse::InternalServerError().body("failed to verify user")
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct AcceptInviteInput {
    pub user_id: UserId,
    pub org_id: OrganizationId,
    pub invite_token: String,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct AcceptInviteResult {
    pub user_id: UserId,
    pub org_id: OrganizationId,
    pub roles: Vec<String>,
}

#[utoipa::path(
    post,
    path = "/auth/accept-invite",
    tag = "Auth",
    request_body = AcceptInviteInput,
    responses(
        (status = 200, description = "Invite accepted", body = AcceptInviteResult),
        (status = 400, description = "Invalid input parameters"),
        (status = 500, description = "Internal server error"),
    )
)]
#[post("/accept-invite")]
pub async fn accept_invite(
    payload: web::Json<AcceptInviteInput>,
    app_state: web::Data<AuthAppData>,
) -> HttpResponse {
    info!("auth accept-invite request received");
    let input = payload.into_inner();

    if input.invite_token.trim().is_empty() {
        warn!("accept-invite rejected: missing invite_token");
        return HttpResponse::BadRequest().body("invite_token is required");
    }

    let params = AcceptInviteParams {
        user_id: input.user_id,
        org_id: input.org_id,
        invite_token: input.invite_token,
    };

    match app_state.auth_provider.accept_invite(params).await {
        Ok(result) => HttpResponse::Ok().json(AcceptInviteResult {
            org_id: result.membership.org_id,
            roles: result.membership.roles,
            user_id: result.membership.user_id,
        }),
        Err(e) => {
            error!(error = %e, "accept-invite failed");
            HttpResponse::InternalServerError().body("failed to accept invite")
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct ResendVerificationInput {
    pub email: String,
}

#[utoipa::path(
    post,
    path = "/auth/resend-verification",
    tag = "Auth",
    request_body = ResendVerificationInput,
    responses(
        (status = 200, description = "Verification email resent"),
        (status = 400, description = "Invalid input parameters"),
        (status = 500, description = "Internal server error"),
    )
)]
#[post("/resend-verification")]
pub async fn resend_verification(
    payload: web::Json<ResendVerificationInput>,
    app_state: web::Data<AuthAppData>,
) -> HttpResponse {
    info!("resend verification request received");
    let input = payload.into_inner();

    if input.email.trim().is_empty() {
        warn!("resend-verification rejected: missing email");
        return HttpResponse::BadRequest().body("email is required");
    }

    match app_state
        .auth_provider
        .resend_verification(input.email)
        .await
    {
        Ok(()) => HttpResponse::Ok().body("verification email sent"),
        Err(e) => {
            error!(error = %e, "resend-verification failed");
            HttpResponse::InternalServerError().body("failed to resend verification email")
        }
    }
}

#[utoipa::path(
    post,
    path = "/auth/logout",
    tag = "Auth",
    responses(
        (status = 200, description = "Logout completed"),
        (status = 500, description = "Internal server error")
    )
)]
#[post("/logout")]
pub async fn logout(app_state: web::Data<AuthAppData>) -> HttpResponse {
    todo!()
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct SendInviteInput {
    pub email: String,
    pub org_id: OrganizationId,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct SendInviteResult {
    pub user_id: UserId,
}

#[utoipa::path(
    post,
    path = "/auth/invite",
    tag = "Auth",
    request_body = SendInviteInput,
    responses(
        (status = 200, description = "Invite sent", body = SendInviteResult),
        (status = 400, description = "Invalid input parameters"),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error"),
    )
)]
#[post("/invite")]
pub async fn invite_user(
    payload: web::Json<SendInviteInput>,
    app_state: web::Data<AuthAppData>,
) -> HttpResponse {
    info!("auth invite-user request received");
    let input = payload.into_inner();

    if input.email.trim().is_empty() {
        warn!("invite-user rejected: missing email");
        return HttpResponse::BadRequest().body("email is required");
    }
    if input.org_id == 0 {
        warn!("invite-user rejected: missing org_id");
        return HttpResponse::BadRequest().body("org_id is required");
    }

    let params = InviteUserParams {
        email: input.email,
        org_id: input.org_id,
    };

    match app_state.auth_provider.invite_user(params).await {
        Ok(result) => HttpResponse::Ok().json(SendInviteResult {
            user_id: result.user_id,
        }),
        Err(ServiceError::NotFound) => HttpResponse::NotFound().body("user not found"),
        Err(e) => {
            error!(error = %e, "invite-user failed");
            HttpResponse::InternalServerError().body("failed to send invite")
        }
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(register_user)
        .service(verify)
        .service(resend_verification)
        .service(accept_invite)
        .service(invite_user)
        .service(logout)
        .service(login);
}
