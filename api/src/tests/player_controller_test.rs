use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse},
    http::Error,
    test, App,
};
use domain::raw_track::RawTrack;
use rstest::rstest;

use crate::controllers::{player_controller, tracks_crud_controller::AddTrackParams};
#[rstest]
#[actix_web::test]
async fn can_play() {}

#[rstest]
#[actix_web::test]
async fn can_pause() -> Result<(), String> {
    let user_name = "some_name";
    let app = test::init_service(App::new().configure(player_controller::init)).await;
    let req = test::TestRequest::get()
        .uri(&format!("/get-user-state/{}", user_name))
        .to_request();
    Ok(())
}

async fn insert(
    user_name: &str,
    track: RawTrack,
    app: &mut impl Service<ServiceRequest, Response = ServiceResponse, Error = Error>,
) {
    let req = test::TestRequest::post()
        .uri("/insert")
        .set_json(AddTrackParams { track: track })
        .to_request();
    todo!();
}
