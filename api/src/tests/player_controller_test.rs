use actix_web::{dev::{Service, ServiceRequest, ServiceResponse}, http::Error, test, App};
use rstest::rstest;

use crate::controllers::player_controller;
#[rstest]
#[actix_web::test]
async fn can_play(){
    
}

#[rstest]
#[actix_web::test]
async fn can_pause()->Result<(),String>{
    let user_name="some_name";
    let app=test::init_service(App::new().configure(player_controller::init)).await;
    let req=test::TestRequest::get().uri(&format!("/get-user-state/{}",user_name)) .to_request();
    Ok(())
} 


async fn insert(
    user_name:&str,
    app: &mut impl Service<Request = ServiceRequest, Response = ServiceResponse, Error = Error>){
    test::call_service(app, req)
}
