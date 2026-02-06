use std::sync::Arc;

use actix_web::{
    get, post,
    web::{self},
    HttpRequest, HttpResponse,
};

use actors::user_actor::create_user_actor_params::CreateUserActorParams;
use serde::{Deserialize, Serialize};
use utoipa::IntoParams;

use crate::{
    app_data::AppData,
    dtos::{claims::Claims, google_user_info::GoogleUserInfo, token_response::TokenResponse},
    token::token_utils::{
        create_access_token, create_refresh_token, generate_csrf_token, verify_token,
    },
};

#[utoipa::path(
    get,
    path = "/auth/google/",
    tag = "auth",
    responses((status = 302, description = "Redirect to Google OAuth"))
)]
#[get("/")]
pub async fn google_auth_redirect() -> HttpResponse {
    println!(
        "🧪 GOOGLE_CLIENT_ID = {:?}",
        std::env::var("GOOGLE_CLIENT_ID")
    );
    let client_id =
        std::env::var("GOOGLE_CLIENT_ID").expect("Could not find GOOGLE_CLIENT_ID in env");
    let redirect_uri =
        std::env::var("GOOGLE_REDIRECT_URI").expect("Could not find GOOGLE_REDIRECT_URI in env");

    let auth_url = format!(
        "https://accounts.google.com/o/oauth2/v2/auth?response_type=code&client_id={}&redirect_uri={}&scope=openid%20email%20profile&access_type=offline&prompt=consent",
        client_id,
        urlencoding::encode(&redirect_uri),
    );

    HttpResponse::Found()
        .append_header(("Location", auth_url))
        .finish()
}

#[utoipa::path(
    get,
    path = "/auth/google/session",
    tag = "auth",
    responses((status = 200, description = "Session info (or null user)", body = serde_json::Value))
)]
#[get("/session")]
pub async fn session(req: HttpRequest, app_data: web::Data<AppData>) -> HttpResponse {
    let Some(auth) = req.cookie("auth_token") else {
        return HttpResponse::Ok().json(serde_json::json!({"user":null}));
    };
    let claims: Claims = match verify_token(auth.value()) {
        Ok(c) => c,
        Err(_) => return HttpResponse::Ok().json(serde_json::json!({"user":null})),
    };
    let user_result = match app_data
        .user_resolver
        .resolve_existing_user_and_actor(&claims.user_id)
        .await
    {
        Ok(u) => u,
        Err(_) => {
            return HttpResponse::NotFound()
                .body(format!("Could not find user with id {}", claims.user_id))
        }
    };
    let user = user_result.user;
    let result = serde_json::json!({
        "user_id":claims.user_id,
        "name":claims.name.unwrap_or_default(),
        "email":claims.email.unwrap_or_default(),
        "photo":user.id
    });
    HttpResponse::Ok().json(serde_json::json!({ "user": result }))
}
#[derive(Deserialize, IntoParams)]
pub(crate) struct AuthRequest {
    code: String,
}

#[derive(Serialize)]
pub struct GoogleLoginResult {
    pub user_id: String,
    pub name: String,
    pub email: String,
    pub picture: String,
}

#[utoipa::path(
    get,
    path = "/auth/google/callback",
    tag = "auth",
    params(AuthRequest),
    responses((status = 200, description = "Login callback (sets cookies)", body = String))
)]
#[get("/callback")]
pub async fn google_callback(
    query: web::Query<AuthRequest>,
    app_data: web::Data<AppData>,
) -> HttpResponse {
    let token_url = "https://oauth2.googleapis.com/token";
    let userinfo_url = "https://www.googleapis.com/oauth2/v3/userinfo";
    match exchange_code_for_user(query.code.clone(), token_url, userinfo_url).await {
        Ok(google_user) => {
            let user_and_actor = match app_data
                .user_resolver
                .resolve_google_user_and_actor(&google_user, |p| {
                    let domain_user_create_params = CreateUserActorParams {
                        user_data: p,
                        user_actor_deps: Arc::clone(&app_data.user_actor_deps),
                    };
                    Ok(domain_user_create_params)
                })
                .await
            {
                Ok(u) => u,
                Err(e) => {
                    eprintln!("User resolution error: {e}");
                    return HttpResponse::InternalServerError().body("User resolution failed");
                }
            };

            let access_token = create_access_token(
                &user_and_actor.user.id,
                Some(&google_user.name),
                Some(google_user.email.as_str()),
                None,
            );
            let refresh_token = create_refresh_token(&user_and_actor.user.id);
            let csrf_token = generate_csrf_token();

            let html = include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/static/postlogin.html"
            ));
            let cookie_settings = if cfg!(debug_assertions) {
                "SameSite=Lax; Path=/"
            } else {
                // For production (HTTPS)
                "SameSite=None; Secure; Path=/"
            };
            HttpResponse::Ok()
                .append_header((
                    "Set-Cookie",
                    format!(
                        "auth_token={}; HttpOnly; {}; Max-Age={}",
                        access_token,
                        cookie_settings,
                        60 * 15
                    ),
                ))
                .append_header((
                    "Set-Cookie",
                    format!(
                        "refresh_token={}; HttpOnly; {}; Max-Age={}",
                        refresh_token,
                        cookie_settings,
                        60 * 60 * 24 * 7
                    ),
                ))
                .append_header((
                    "Set-Cookie",
                    format!(
                        "csrf_token={}; {}; Max-Age={}",
                        csrf_token,
                        cookie_settings,
                        60 * 15
                    ),
                ))
                .content_type("text/html; charset=utf-8")
                .body(html)
        }
        Err(err) => {
            eprintln!("OAuth error: {err}");
            HttpResponse::InternalServerError().body("OAuth failed")
        }
    }
}

#[utoipa::path(
    post,
    path = "/auth/google/refresh",
    tag = "auth",
    responses((status = 200, description = "Refresh auth cookie", body = serde_json::Value))
)]
#[post("/refresh")]
pub async fn refresh(req: HttpRequest) -> HttpResponse {
    let refresh_cookie = match req.cookie("refresh_token") {
        Some(cookie) => cookie,
        None => return HttpResponse::Unauthorized().body("Missing refresh token"),
    };
    let claims = match verify_token(refresh_cookie.value()) {
        Ok(claims) => claims,
        Err(_) => return HttpResponse::Unauthorized().body("Invalid refresh token"),
    };
    let user_id = claims.user_id;

    let email = claims.email.unwrap_or_default();

    let new_access_token =
        create_access_token(&user_id, claims.name.as_deref(), Some(&email), None);
    let new_crsf_token = generate_csrf_token();
    HttpResponse::Ok()
        .append_header((
            "Set-Cookie",
            format!(
                "auth_token={}; HttpOnly; SameSite=None; Path=/; Max-Age={}",
                new_access_token,
                60 * 15
            ),
        ))
        .append_header((
            "Set-Cookie",
            format!(
                "csrf_token={}; SameSite=Lax; Path=/; Max-Age={}",
                new_crsf_token,
                60 * 15
            ),
        ))
        .json(serde_json::json!({"status":"refreshed"}))
}
#[utoipa::path(
    post,
    path = "/auth/google/logout",
    tag = "auth",
    responses((status = 302, description = "Logout (clears cookies and redirects)"))
)]
#[post("/logout")]
pub async fn logout() -> HttpResponse {
    HttpResponse::Found()
        .append_header((
            "Set-Cookie",
            "auth_token=deleted; Path=/; HttpOnly; Max-Age=0",
        ))
        .append_header((
            "Set-Cookie",
            "refresh_token=deleted; Path=/; HttpOnly; Max-Age=0",
        ))
        .append_header(("Location", "/"))
        .body("Logged out")
}

pub(crate) async fn exchange_code_for_user(
    code: String,
    token_url: &str,
    userinfo_url: &str,
) -> Result<GoogleUserInfo, String> {
    let client_id = std::env::var("GOOGLE_CLIENT_ID").unwrap();
    let client_secret = std::env::var("GOOGLE_CLIENT_SECRET").unwrap();
    let redirect_uri = std::env::var("GOOGLE_REDIRECT_URI").unwrap();
    let params = [
        ("code", code),
        ("client_id", client_id),
        ("client_secret", client_secret),
        ("redirect_uri", redirect_uri),
        ("grant_type", "authorization_code".to_string()),
    ];

    let token_res = reqwest::Client::new()
        .post(token_url)
        .form(&params)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let token = token_res
        .json::<TokenResponse>()
        .await
        .map_err(|e| e.to_string())?;
    let user_res = reqwest::Client::new()
        .get(userinfo_url)
        .bearer_auth(&token.access_token)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let user = user_res
        .json::<GoogleUserInfo>()
        .await
        .map_err(|e| e.to_string())?;
    Ok(user)
}
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(google_auth_redirect)
        .service(google_callback)
        .service(logout)
        .service(refresh)
        .service(session);
}
