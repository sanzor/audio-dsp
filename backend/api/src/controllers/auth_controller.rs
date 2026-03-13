use actix_web::{post, web, HttpResponse};
use domain::project_role::ProjectRole;
use serde::{Deserialize, Serialize};
use tracing::{error, info, warn};
use utoipa::ToSchema;

use crate::auth::{
    accept_invite_params::AcceptInviteParams,
    auth_app_data::AuthAppData,
    invite_user_params::InviteUserParams,
    login_params::LoginParams,
    register_user_params::RegisterUserParams,
    service_error::ServiceError,
    user::AuthUser,
    verify_user_params::VerifyUserParams,
};

// ── login ─────────────────────────────────────────────────────────────────────

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct LoginInput {
    pub email: String,
    pub password_hash: String,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct LoginOutput {
    pub user: AuthUser,
    pub token: String,
}

#[utoipa::path(post, path = "/auth/login", tag = "Auth",
    request_body = LoginInput,
    responses((status = 200, body = LoginOutput), (status = 404)))]
#[post("/login")]
pub async fn login(
    payload: web::Json<LoginInput>,
    app_state: web::Data<AuthAppData>,
) -> HttpResponse {
    info!("auth login");
    let input = payload.into_inner();
    if input.email.trim().is_empty() { return HttpResponse::BadRequest().body("email required"); }
    if input.password_hash.trim().is_empty() { return HttpResponse::BadRequest().body("password_hash required"); }

    match app_state.auth_provider.login(LoginParams { email: input.email, password_hash: input.password_hash }).await {
        Ok(r) => HttpResponse::Ok().json(LoginOutput { user: r.user, token: r.token }),
        Err(ServiceError::NotFound) => HttpResponse::NotFound().body("invalid credentials"),
        Err(e) => { error!(error = %e, "login failed"); HttpResponse::InternalServerError().body("login failed") }
    }
}

// ── register ──────────────────────────────────────────────────────────────────

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct RegisterInput {
    pub email: String,
    pub password_hash: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct RegisterOutput {
    pub user: AuthUser,
    pub token: String,
}

#[utoipa::path(post, path = "/auth/register", tag = "Auth",
    request_body = RegisterInput,
    responses((status = 201, body = RegisterOutput)))]
#[post("/register")]
pub async fn register(
    payload: web::Json<RegisterInput>,
    app_state: web::Data<AuthAppData>,
) -> HttpResponse {
    info!("auth register");
    let input = payload.into_inner();
    if input.email.trim().is_empty() { return HttpResponse::BadRequest().body("email required"); }
    if input.password_hash.trim().is_empty() { return HttpResponse::BadRequest().body("password_hash required"); }
    if input.name.trim().is_empty() { return HttpResponse::BadRequest().body("name required"); }

    match app_state.auth_provider.register(RegisterUserParams {
        email: input.email, password_hash: input.password_hash, name: input.name,
    }).await {
        Ok(r) => HttpResponse::Created().json(RegisterOutput { user: r.user, token: r.token }),
        Err(e) => { error!(error = %e, "register failed"); HttpResponse::InternalServerError().body("register failed") }
    }
}

// ── verify ────────────────────────────────────────────────────────────────────

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct VerifyInput { pub token: String }

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct VerifyOutput { pub user: AuthUser }

#[utoipa::path(post, path = "/auth/verify", tag = "Auth",
    request_body = VerifyInput,
    responses((status = 200, body = VerifyOutput), (status = 400)))]
#[post("/verify")]
pub async fn verify(
    payload: web::Json<VerifyInput>,
    app_state: web::Data<AuthAppData>,
) -> HttpResponse {
    info!("auth verify");
    let input = payload.into_inner();
    if input.token.trim().is_empty() { return HttpResponse::BadRequest().body("token required"); }

    let claims = match app_state.jwt_provider.verify(&input.token) {
        Ok(c) => c,
        Err(e) => { warn!(error = %e, "invalid verify token"); return HttpResponse::BadRequest().body("invalid or expired token"); }
    };

    if claims.purpose.as_deref() != Some("verification") {
        return HttpResponse::BadRequest().body("invalid token purpose");
    }

    match app_state.auth_provider.verify(VerifyUserParams { user_id: claims.user_id }).await {
        Ok(r) => HttpResponse::Ok().json(VerifyOutput { user: r.user }),
        Err(e) => { error!(error = %e, "verify failed"); HttpResponse::InternalServerError().body("verify failed") }
    }
}

// ── resend-verification ───────────────────────────────────────────────────────

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct ResendInput { pub email: String }

#[utoipa::path(post, path = "/auth/resend-verification", tag = "Auth",
    request_body = ResendInput,
    responses((status = 200), (status = 400)))]
#[post("/resend-verification")]
pub async fn resend_verification(
    payload: web::Json<ResendInput>,
    app_state: web::Data<AuthAppData>,
) -> HttpResponse {
    info!("resend verification");
    let input = payload.into_inner();
    if input.email.trim().is_empty() { return HttpResponse::BadRequest().body("email required"); }

    match app_state.auth_provider.resend_verification(input.email).await {
        Ok(()) => HttpResponse::Ok().body("verification email sent"),
        Err(e) => { error!(error = %e, "resend failed"); HttpResponse::InternalServerError().body("resend failed") }
    }
}

// ── invite ────────────────────────────────────────────────────────────────────

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct InviteInput {
    pub email: String,
    pub project_id: String,
    pub role: ProjectRole,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct InviteOutput { pub user_id: String }

#[utoipa::path(post, path = "/auth/invite", tag = "Auth",
    request_body = InviteInput,
    responses((status = 200, body = InviteOutput), (status = 404)))]
#[post("/invite")]
pub async fn invite_user(
    payload: web::Json<InviteInput>,
    app_state: web::Data<AuthAppData>,
) -> HttpResponse {
    info!("auth invite");
    let input = payload.into_inner();
    if input.email.trim().is_empty() { return HttpResponse::BadRequest().body("email required"); }
    if input.project_id.trim().is_empty() { return HttpResponse::BadRequest().body("project_id required"); }

    match app_state.auth_provider.invite_user(InviteUserParams {
        email: input.email,
        project_id: input.project_id,
        role: input.role,
    }).await {
        Ok(r) => HttpResponse::Ok().json(InviteOutput { user_id: r.user_id }),
        Err(ServiceError::NotFound) => HttpResponse::NotFound().body("user not found"),
        Err(e) => { error!(error = %e, "invite failed"); HttpResponse::InternalServerError().body("invite failed") }
    }
}

// ── accept-invite ─────────────────────────────────────────────────────────────

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct AcceptInviteInput { pub invite_token: String }

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct AcceptInviteOutput {
    pub user_id: String,
    pub project_id: String,
    pub role: ProjectRole,
    pub token: String,
}

#[utoipa::path(post, path = "/auth/accept-invite", tag = "Auth",
    request_body = AcceptInviteInput,
    responses((status = 200, body = AcceptInviteOutput)))]
#[post("/accept-invite")]
pub async fn accept_invite(
    payload: web::Json<AcceptInviteInput>,
    app_state: web::Data<AuthAppData>,
) -> HttpResponse {
    info!("auth accept-invite");
    let input = payload.into_inner();
    if input.invite_token.trim().is_empty() { return HttpResponse::BadRequest().body("invite_token required"); }

    match app_state.auth_provider.accept_invite(AcceptInviteParams { invite_token: input.invite_token }).await {
        Ok(r) => HttpResponse::Ok().json(AcceptInviteOutput {
            user_id: r.user_id,
            project_id: r.project_id,
            role: r.role,
            token: r.token,
        }),
        Err(e) => { error!(error = %e, "accept-invite failed"); HttpResponse::InternalServerError().body("accept-invite failed") }
    }
}

// ── logout ────────────────────────────────────────────────────────────────────

#[utoipa::path(post, path = "/auth/logout", tag = "Auth",
    responses((status = 200)))]
#[post("/logout")]
pub async fn logout() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(login)
        .service(register)
        .service(verify)
        .service(resend_verification)
        .service(invite_user)
        .service(accept_invite)
        .service(logout);
}
