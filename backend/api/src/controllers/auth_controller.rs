use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use tracing::{error, info, warn};
use utoipa::ToSchema;

use crate::{
    auth::{
        auth_app_data::AuthAppData,
        login_params::LoginParams,
        register_user_params::RegisterUserParams,
        service_error::ServiceError,
        user::AuthUser,
        verify_user_params::VerifyUserParams,
    },
    token::token_utils::{create_access_token, create_refresh_token, verify_token},
};

fn cookie_settings() -> &'static str {
    if cfg!(debug_assertions) { "SameSite=Lax; Path=/" } else { "SameSite=None; Secure; Path=/" }
}

fn set_auth_cookie(token: &str) -> String {
    format!("auth_token={}; HttpOnly; {}; Max-Age={}", token, cookie_settings(), 60 * 15)
}

fn set_refresh_cookie(token: &str) -> String {
    format!("refresh_token={}; HttpOnly; {}; Max-Age={}", token, cookie_settings(), 60 * 60 * 24 * 7)
}

fn clear_auth_cookie() -> &'static str {
    "auth_token=deleted; HttpOnly; Path=/; Max-Age=0"
}

fn clear_refresh_cookie() -> &'static str {
    "refresh_token=deleted; HttpOnly; Path=/; Max-Age=0"
}

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
    responses((status = 200, body = LoginOutput), (status = 401)))]
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
        Ok(r) => {
            let refresh_tok = create_refresh_token(r.user.id, Some(&r.user.name), Some(&r.user.email), r.user.is_admin);
            HttpResponse::Ok()
                .append_header(("Set-Cookie", set_auth_cookie(&r.token)))
                .append_header(("Set-Cookie", set_refresh_cookie(&refresh_tok)))
                .json(LoginOutput { user: r.user, token: r.token })
        },
        Err(ServiceError::NotFound) => HttpResponse::Unauthorized().body("invalid credentials"),
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
        Ok(r) => {
            let refresh_tok = create_refresh_token(r.user.id, Some(&r.user.name), Some(&r.user.email), r.user.is_admin);
            HttpResponse::Created()
                .append_header(("Set-Cookie", set_auth_cookie(&r.token)))
                .append_header(("Set-Cookie", set_refresh_cookie(&refresh_tok)))
                .json(RegisterOutput { user: r.user, token: r.token })
        },
        Err(ServiceError::Conflict(msg)) => HttpResponse::Conflict().body(msg),
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

// ── refresh ───────────────────────────────────────────────────────────────────

#[derive(Serialize, ToSchema)]
pub struct RefreshOutput {
    pub token: String,
}

#[utoipa::path(post, path = "/auth/refresh", tag = "Auth",
    responses((status = 200, body = RefreshOutput), (status = 401)))]
#[post("/refresh")]
pub async fn refresh(req: actix_web::HttpRequest) -> HttpResponse {
    let refresh_token = match req.cookie("refresh_token") {
        Some(c) => c.value().to_owned(),
        None => return HttpResponse::Unauthorized().body("missing refresh token"),
    };

    let claims = match verify_token(&refresh_token) {
        Ok(c) => c,
        Err(_) => return HttpResponse::Unauthorized().body("invalid or expired refresh token"),
    };

    if claims.purpose.as_deref() != Some("refresh") {
        return HttpResponse::Unauthorized().body("invalid token purpose");
    }

    let new_access = create_access_token(
        claims.user_id,
        claims.name.as_deref(),
        claims.email.as_deref(),
        claims.is_admin,
    );
    let new_refresh = create_refresh_token(
        claims.user_id,
        claims.name.as_deref(),
        claims.email.as_deref(),
        claims.is_admin,
    );

    HttpResponse::Ok()
        .append_header(("Set-Cookie", set_auth_cookie(&new_access)))
        .append_header(("Set-Cookie", set_refresh_cookie(&new_refresh)))
        .json(RefreshOutput { token: new_access })
}

// ── logout ────────────────────────────────────────────────────────────────────

#[utoipa::path(post, path = "/auth/logout", tag = "Auth", responses((status = 200)))]
#[post("/logout")]
pub async fn logout() -> HttpResponse {
    HttpResponse::Ok()
        .append_header(("Set-Cookie", clear_auth_cookie()))
        .append_header(("Set-Cookie", clear_refresh_cookie()))
        .finish()
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(login)
        .service(register)
        .service(verify)
        .service(resend_verification)
        .service(refresh)
        .service(logout);
}
