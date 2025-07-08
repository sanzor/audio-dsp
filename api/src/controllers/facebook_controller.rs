use actix_web::{get, web};

#[get("/callback")]
async fn facebook_login(query:web::Query<std::collections::HashMap<String,String>>)->HttpResponse{
    let code=query.get("code").unwrap().to_string();
    let client=create_facebook_client();
    // let token_result=client.set
    todo!()

}
pub fn init(cfg:&mut web::ServiceConfig){
    cfg.service(google_login)

}