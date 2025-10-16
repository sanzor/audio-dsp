use crate::controllers::player_controller;
use actix_web::{
    test::{self},
    App,
};
use rstest::rstest;
#[rstest]
#[actix_web::test]
async fn can_play() {}

#[rstest]
#[actix_web::test]
async fn can_pause() -> Result<(), String> {
    let user_name = "some_name";
    let _app = test::init_service(App::new().configure(player_controller::init)).await;
    let _req = test::TestRequest::get()
        .uri(&format!("/get-user-state/{}", user_name))
        .to_request();
    Ok(())
}
