use actix_web::{get, web, HttpResponse};

use domain::domain_user::DomainUser;
use serde::{Deserialize, Serialize};

use crate::{app_data::AppData, user_provider::create_user_params::CreateUserParams};

#[get("/auth/google")]
async fn google_auth_redirect() -> HttpResponse {
    let client_id = std::env::var("GOOGLE_CLIENT_ID").unwrap();
    let redirect_uri = std::env::var("GOOGLE_REDIRECT_URI").unwrap();

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

#[derive(Deserialize)]
pub(crate) struct TokenResponse {
    access_token: String,
    id_token: Option<String>,
    expires_in: u64,
    token_type: String,
    refresh_token: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct GoogleUserInfo {
    pub sub: String,
    pub email: String,
    pub name: String,
    pub picture: String,
}

#[get("/callback")]
async fn google_callback(query: web::Query<AuthRequest>,app_data:web::Data<AppData>) -> HttpResponse {
    let token_url = "https://oauth2.googleapis.com/token";
    let userinfo_url = "https://www.googleapis.com/oauth2/v3/userinfo";
    match exchange_code_for_user(query.code.clone(), token_url, userinfo_url).await {
        Ok(google_user) =>{
            let google_sub_id=google_user.sub.clone();
            if let Some(user)=app_data.user_provider.get_user_by_google_sub_id(&google_sub_id).await{
                
            }else{
                let domain_user_create_params=CreateUserParams{
                    email:google_user.email,
                    name:google_user.name,
                    picture:google_user.picture,
                    google_sub_id:Some(google_user.sub)
                };
                let insert_result=app_data.user_provider.create_user(domain_user_create_params).await;
            }
            HttpResponse::Ok().json(google_user)
        }
        Err(err) => {
            eprintln!("OAuth error: {}", err);
            HttpResponse::InternalServerError().body("OAuth failed")
        }
    }
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
    cfg.service(google_auth_redirect).service(google_callback);
}
