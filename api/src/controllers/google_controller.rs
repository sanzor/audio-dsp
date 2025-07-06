use actix_web::{web, HttpResponse};
use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, TokenUrl};

#[derive(Deserialize)]
struct GoogleUserInfo {
    sub: String,
    email: String,
    name: String,
    picture: String,
}

fn create_google_client()->BasicClient{
    let client_id = ClientId::new(env::var("GOOGLE_CLIENT_ID").unwrap());
    let client_secret = ClientSecret::new(env::var("GOOGLE_CLIENT_SECRET").unwrap());
    let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/auth".to_string()).unwrap();
    let token_url = TokenUrl::new("https://oauth2.googleapis.com/token".to_string()).unwrap();
     BasicClient::new(client_id)
        .set_redirect_uri(RedirectUrl::new("http://localhost:8000/auth/google/callback".to_string()).unwrap())
}

#[get("/callback")]
async fn google_login(query:web::Query<std::collections::HashMap<String,String>>)->HttpResponse{
    let code=query.get("code").unwrap().to_string();
    let client=create_google_client();
    // let token_result=client
    todo!()

}
pub fn init(cfg:&mut web::ServiceConfig){
    cfg.service(google_login)

}