

use actix_web::{get, web, HttpResponse};

use serde::{Deserialize, Serialize};



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
struct AuthRequest {
    code: String,
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    id_token: Option<String>,
    expires_in: u64,
    token_type: String,
    refresh_token: Option<String>,
}

#[derive(Deserialize,Serialize)]
struct GoogleUserInfo {
    sub: String,
    email: String,
    name: String,
    picture: String,
}

#[get("/auth/google/callback")]
async fn google_callback(query: web::Query<AuthRequest>) -> HttpResponse {
    let client_id = std::env::var("GOOGLE_CLIENT_ID").unwrap();
    let client_secret = std::env::var("GOOGLE_CLIENT_SECRET").unwrap();
    let redirect_uri = std::env::var("GOOGLE_REDIRECT_URI").unwrap();

    let params = [
        ("code", query.code.clone()),
        ("client_id", client_id),
        ("client_secret", client_secret),
        ("redirect_uri", redirect_uri),
        ("grant_type", "authorization_code".to_string()),
    ];

    let token_res = reqwest::Client::new()
        .post("https://oauth2.googleapis.com/token")
        .form(&params)
        .send()
        .await;

    if let Ok(res) = token_res {
        if let Ok(token) = res.json::<TokenResponse>().await {
            let user_res = reqwest::Client::new()
                .get("https://www.googleapis.com/oauth2/v3/userinfo")
                .bearer_auth(&token.access_token)
                .send()
                .await;

            if let Ok(resp) = user_res {
                if let Ok(user) = resp.json::<GoogleUserInfo>().await {
                    return HttpResponse::Ok().json(user);
                }
            }
        }
    }

    HttpResponse::InternalServerError().body("OAuth failed")
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg
        .service(google_auth_redirect)
        .service(google_callback);
}