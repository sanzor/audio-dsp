use actix_web::{get, web, HttpResponse};

#[utoipa::path(
    get,
    path = "/auth/facebook/callback",
    tag = "auth",
    responses((status = 501, description = "Not implemented"))
)]
#[get("/callback")]
pub async fn facebook_login(
    query: web::Query<std::collections::HashMap<String, String>>,
) -> HttpResponse {
    let _code = query.get("code").unwrap().to_string();
    // let client=create_facebook_client();
    // let token_result=client.set
    HttpResponse::NotImplemented().finish()
}
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(facebook_login);
}
