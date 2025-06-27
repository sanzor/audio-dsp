use actix_web::{test, App};
use rstest::rstest;

use crate::controllers::player_controller;
#[rstest]
#[actix_web::test]
async fn can_play(){
    
}

#[rstest]
#[actix_web::test]
async fn can_pause()->Result<(),String>{
    let app=test::init_service(App::new().configure(player_controller::init)).await;
    let req=test::TestRequest::get()
    Ok(())
}