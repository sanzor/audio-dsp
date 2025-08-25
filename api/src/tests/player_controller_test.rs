use crate::controllers::player_controller;
use actix_web::http::header::CONTENT_TYPE;
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse},
    http::Error,
    test::{self, TestRequest},
    web::BytesMut,
    App,
};
use domain::raw_track::RawTrack;
use rstest::rstest;
use std::fmt::Write as _;
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
    let boundary = "----testboundary";
    let mut body = BytesMut::new();

    // write text fields (no macro)
    let mut write_field = |name: &str, value: &str| {
        let mut part = String::new();
        write!(
            &mut part,
            "--{}\r\nContent-Disposition: form-data; name=\"{}\"\r\n\r\n{}\r\n",
            boundary, name, value
        )
        .unwrap();
        body.extend_from_slice(part.as_bytes());
    };

    write_field("name", &track.info.name);
    write_field("extension", &track.info.extension);
    write_field("sample_rate", &track.data.sample_rate.to_string());

    // if Channels is an enum, prefer sending a numeric count (implement .as_u8() if needed)
    write_field("channels", &format!("{:?}", track.data.channels)); // or &track.data.channels.as_u8().to_string()

    // samples: Vec<f32> -> bytes (LE)
    let mut sample_bytes = Vec::with_capacity(track.data.samples.len() * 4);
    for &s in &track.data.samples {
        sample_bytes.extend_from_slice(&s.to_le_bytes());
    }

    // binary part
    let mut bin_hdr = String::new();
    write!(
        &mut bin_hdr,
        "--{}\r\nContent-Disposition: form-data; name=\"samples\"; filename=\"samples.bin\"\
\r\nContent-Type: application/octet-stream\r\n\r\n",
        boundary
    )
    .unwrap();
    body.extend_from_slice(bin_hdr.as_bytes());
    body.extend_from_slice(&sample_bytes);
    body.extend_from_slice(b"\r\n");

    // closing boundary
    let end = format!("--{}--\r\n", boundary);
    body.extend_from_slice(end.as_bytes());

    // Build a ServiceRequest directly:
    let srv_req = test::TestRequest::post()
        .uri("/insert")
        .insert_header((
            CONTENT_TYPE,
            format!("multipart/form-data; boundary={}", boundary),
        ))
        .set_payload(body.freeze())
        .to_srv_request(); // <-- this returns ServiceRequest

    // Call the service
    let _resp = app.call(srv_req).await.unwrap();
}
