use actix_web::{test, App};
use audiolib::{audio_buffer::AudioBuffer, Channels};
use domain::raw_track::{RawTrack, TrackInfo};
use rstest::rstest;

use crate::controllers::tracks_crud_controller::{self, AddTrackParams, AddTrackResult};

#[rstest]
#[actix_web::test]
async fn can_insert_track() -> Result<(), String> {
    let track = make_track_from_samples(vec![1_f32; 500], Channels::Mono);
    let user_name = "some_name";

    let app = test::init_service(App::new().configure(tracks_crud_controller::init)).await;

    let req = test::TestRequest::post()
        .uri("/tracks/add-track")
        .set_json(&AddTrackParams {
            track,
            user_id: user_name.to_string(),
        })
        .to_request();

    // let resp: AddTrackResult = test::call_and_read_body_json(&app, req).await;

    Ok(())
}

fn make_track_from_samples(samples: Vec<f32>, channels: Channels) -> RawTrack {
    match channels {
        Channels::Mono => RawTrack {
            info: TrackInfo {
                name: "some_name".to_string(),
            },
            data: AudioBuffer {
                channels: Channels::Mono,
                sample_rate: 1_f32,
                samples: samples.clone(),
            },
        },
        Channels::Stereo => RawTrack {
            info: TrackInfo {
                name: "some_name".to_string(),
            },
            data: AudioBuffer {
                samples: samples.clone(),
                sample_rate: 1_f32,
                channels: Channels::Stereo,
            },
        },
    }
}
