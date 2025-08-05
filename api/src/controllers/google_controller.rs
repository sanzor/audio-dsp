use std::{env, sync::Arc};

use actix_web::{
    get, post, web::{self, Json}, HttpRequest, HttpResponse
};

use actors::user_actor::create_user_actor_params::CreateUserActorParams;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::{
    app_data::AppData,
    dtos::{claims::Claims, google_user_info::GoogleUserInfo, token_response::TokenResponse},
    token::{csrf_token, token_utils::{create_access_token, create_refresh_token, create_token, generate_csrf_token, verify_token}},
};

#[get("/")]
async fn google_auth_redirect() -> HttpResponse {
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
#[derive(Deserialize)]
pub(crate) struct AuthRequest {
    code: String,
}

#[derive(Serialize)]
pub struct GoogleLoginResult {
    pub user_id: String,
    pub name:String,
    pub email: String,
    pub picture: String,
}

#[get("/callback")]
async fn google_callback(
    query: web::Query<AuthRequest>,
    app_data: web::Data<AppData>,
) -> HttpResponse {
    let token_url = "https://oauth2.googleapis.com/token";
    let userinfo_url = "https://www.googleapis.com/oauth2/v3/userinfo";
    match exchange_code_for_user(query.code.clone(), token_url, userinfo_url).await {
        Ok(google_user) => {
            let resolved_user = match app_data
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
                    eprintln!("User resolution error: {}", e);
                    return HttpResponse::InternalServerError().body("User resolution failed");
                }
            };

            let access_token =
                create_access_token(&google_user.sub, Some(google_user.email.as_str()), None);
            let refresh_token = create_refresh_token(&google_user.sub);
            let csrf_token=generate_csrf_token();

            HttpResponse::Found()
                .append_header((
                    "Set-Cookie",
                    format!(
                        "auth_token={}; HttpOnly; SameSite=Lax; Path=/; Max-Age={}",
                        access_token,
                        60 * 15
                    ),
                ))
                .append_header((
                    "Set-Cookie",
                    format!(
                        "refresh_token={}; HttpOnly; SameSite=Lax; Path=/; Max-Age={}",
                        refresh_token,
                        60 * 60 * 24 * 7
                    ),
                ))
                .append_header(("Set-Cookie",format!("csrf_token={}; SameSite=Lax; Path=/; Max-Age={}",csrf_token,60*15)))
                .append_header(("Location", "/"))
                .json(GoogleLoginResult {
                    email: resolved_user.domain_user.email,
                    name:resolved_user.domain_user.name,    
                    user_id: resolved_user.domain_user.id,
                    picture: resolved_user.domain_user.picture,
                })
        }
        Err(err) => {
            eprintln!("OAuth error: {}", err);
            HttpResponse::InternalServerError().body("OAuth failed")
        }
    }
}

#[post("/refresh")]
async fn refresh(req:HttpRequest)->HttpResponse{
    let refresh_cookie=req.cookie("refresh_token");
    if let Some(cookie)=refresh_cookie{
        match verify_token()
    }
}
#[post("/logout")]
async fn logout() -> HttpResponse {
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
        .service(logout);
}
