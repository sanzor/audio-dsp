use std::str::FromStr;

use crate::{
    app_data::AppData, controllers::utils::get_user_actor_internal,
    dtos::authenticated_user::AuthenticatedUser,
};
use actix_multipart::Multipart;
use actix_web::{
    delete, get, post,
    web::{self},
    HttpResponse,
};
use audiolib::{audio_buffer::AudioBuffer, Channels};
use domain::{
    actors::messages::crud::{
        copy_track::CopyTrack, delete_track::DeleteTrack, get_track::GetRawTrack,
        get_track_info::GetTrackMeta, get_tracks::GetTrackMetas, insert_track::InsertTrack,
        update_track_info::UpdateTrackInfo,
    },
    raw_track::{RawTrack, TrackInfo},
};
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct AddTrackParams {
    pub track: RawTrack,
}

#[derive(Serialize, Deserialize)]
pub struct AddTrackResult {
    pub track_id: String,
}

#[post("/add-track")]
async fn add_track(
    user: AuthenticatedUser,
    path: web::Json<AddTrackParams>,
    app_state: web::Data<AppData>,
) -> HttpResponse {
    let request = path.into_inner();

    let found_user = match get_user_actor_internal(&user.user_id, &app_state).await {
        Ok(u) => u,
        Err(e) => return HttpResponse::NotFound().body("User not found"),
    };

    let rez = match found_user
        .ask(InsertTrack {
            track: request.track,
        })
        .await
    {
        Ok(smth) => HttpResponse::Ok().json(AddTrackResult {
            track_id: smth.track_id,
        }),
        Err(e) => return HttpResponse::InternalServerError().body("Could not insert track"),
    };
    rez
}

#[post("/add-track-multi")]
async fn add_track_multi(
    user: AuthenticatedUser,
    mut payload: Multipart,
    app_state: web::Data<AppData>,
) -> HttpResponse {
    let mut name: Option<String> = None;
    let mut extension: Option<String> = None;
    let mut sample_rate: Option<f32> = None; // ✅ f32 now
    let mut channels: Option<Channels> = None; // ✅ Enum now
    let mut samples_bytes: Vec<u8> = vec![];

    while let Some(Ok(mut field)) = payload.next().await {
        let field_name = field.name().unwrap_or("").to_string();
        let field_data = next_multipart_field(&mut field).await;

        match field_name.as_str() {
            "name" => name = Some(String::from_utf8_lossy(&field_data).to_string()),
            "extension" => extension = Some(String::from_utf8_lossy(&field_data).to_string()),
            "sample_rate" => sample_rate = String::from_utf8_lossy(&field_data).parse::<f32>().ok(),
            "channels" => channels = Channels::from_str(&String::from_utf8_lossy(&field_data)).ok(),
            "samples" => samples_bytes = field_data,
            _ => {}
        }
    }
    let (name, extension, sample_rate, channels) = match (name, extension, sample_rate, channels) {
        (Some(n), Some(ext), Some(sr), Some(ch)) => (n, ext, sr, ch),
        _ => return HttpResponse::BadRequest().body("Missing required fields"),
    };
    if samples_bytes.is_empty() {
        return HttpResponse::BadRequest().body("Missing samples data");
    };
    let samples: Vec<f32> = bytes_to_f32(samples_bytes);

    let audio_buffer = AudioBuffer {
        samples,
        sample_rate,
        channels,
    };

    let raw_track = RawTrack {
        info: TrackInfo { name, extension },
        data: audio_buffer,
    };

    // ✅ Continue same as before
    let found_user = match get_user_actor_internal(&user.user_id, &app_state).await {
        Ok(u) => u,
        Err(_) => return HttpResponse::NotFound().body("User not found"),
    };

    let result = found_user.ask(InsertTrack { track: raw_track }).await;

    match result {
        Ok(smth) => HttpResponse::Ok().json(AddTrackResult {
            track_id: smth.track_id,
        }),
        Err(_) => HttpResponse::InternalServerError().body("Could not insert track"),
    }
}

#[derive(Deserialize)]
pub struct CopyTrackParams {
    pub track_id: String,
    pub copy_track_name: String,
}

#[post("/copy-track")]
async fn copy_track(
    user: AuthenticatedUser,
    request_raw: web::Json<CopyTrackParams>,
    app_state: web::Data<AppData>,
) -> HttpResponse {
    let request = request_raw.into_inner();

    let user = match get_user_actor_internal(&user.user_id, &app_state).await {
        Ok(u) => u,
        Err(e) => return HttpResponse::NotFound().body("User not found"),
    };

    let rez = match user
        .ask(CopyTrack {
            track_id: request.track_id,
            track_copy_name: request.copy_track_name,
        })
        .await
    {
        Ok(smth) => HttpResponse::Ok().json("track copied"),
        Err(e) => 
        
        
        return HttpResponse::InternalServerError().body("Could not copy track"),
    };
    rez
}

#[derive(Deserialize)]
pub struct UpdateTrackParams {
    pub track_id: String,
    pub track_name: String
}
#[post("/update-track-info")]
async fn update_track_info(
    user: AuthenticatedUser,
    path: web::Json<UpdateTrackParams>,
    app_state: web::Data<AppData>,
) -> HttpResponse {
    let request = path.into_inner();

    let user = match get_user_actor_internal(&user.user_id, &app_state).await {
        Ok(u) => u,
        Err(e) => return HttpResponse::NotFound().body("User not found"),
    };

    let rez = match user
        .ask(UpdateTrackInfo {
            name:request.track_name,
            track_id: request.track_id
        })
        .await
    {
        Ok(smth) => HttpResponse::Ok().json("track updated"),
        Err(e) => return HttpResponse::InternalServerError().body("Could not insert track"),
    };
    rez
}
#[derive(Deserialize)]
pub struct RemoveTrackParams {
    pub track_id: String,
}

#[delete("/remove")]
async fn remove_track(
    user: AuthenticatedUser,
    path: web::Query<RemoveTrackParams>,
    app_state: web::Data<AppData>,
) -> HttpResponse {
    let request = path.into_inner();

    let user = match get_user_actor_internal(&user.user_id, &app_state).await {
        Ok(u) => u,
        Err(e) => return HttpResponse::NotFound().body("User not found"),
    };

    let rez = match user
        .ask(DeleteTrack {
            track_id: request.track_id,
        })
        .await
    {
        Ok(smth) => HttpResponse::Ok().json("track added"),
        Err(e) => return HttpResponse::InternalServerError().body("Could not remove track"),
    };
    rez
}
#[derive(Deserialize)]
pub struct GetTrackParams {
    pub track_id: String,
}
#[get("/get-raw")]
async fn get_raw(
    user: AuthenticatedUser,
    query: web::Query<GetTrackParams>,
    app_state: web::Data<AppData>,
) -> HttpResponse {
    let request = query.into_inner();

    let user = match get_user_actor_internal(&user.user_id, &app_state).await {
        Ok(u) => u,
        Err(e) => return HttpResponse::NotFound().body("User not found"),
    };

    let rez = match user
        .ask(GetRawTrack {
            track_id: request.track_id,
        })
        .await
    {
        Ok(smth) => HttpResponse::Ok().json(smth),
        Err(e) => return HttpResponse::InternalServerError().body("Could not get track"),
    };
    rez
}

#[get("/get-meta")]
async fn get_meta(
    user: AuthenticatedUser,
    query: web::Query<GetTrackParams>,
    app_state: web::Data<AppData>,
) -> HttpResponse {
    let request = query.into_inner();

    let user = match get_user_actor_internal(&user.user_id, &app_state).await {
        Ok(u) => u,
        Err(e) => return HttpResponse::NotFound().body("User not found"),
    };

    let rez = match user
        .ask(GetTrackMeta {
            track_id: request.track_id,
        })
        .await
    {
        Ok(smth) => HttpResponse::Ok().json(smth),
        Err(e) => return HttpResponse::InternalServerError().body("Could not get track"),
    };
    rez
}

#[get("/get-all")]
async fn get_tracks(user: AuthenticatedUser, app_state: web::Data<AppData>) -> HttpResponse {
    let user = match get_user_actor_internal(&user.user_id, &app_state).await {
        Ok(u) => u,
        Err(e) => return HttpResponse::NotFound().body("User not found"),
    };

    let rez = match user.ask(GetTrackMetas {}).await {
        Ok(smth) => HttpResponse::Ok().json(smth),
        Err(e) => return HttpResponse::InternalServerError().body("Could not get tracks"),
    };
    rez
}

#[derive(Deserialize)]
pub struct GetTrackInfoParams {
    pub track_id: String,
}
#[get("/get-track-info")]
async fn get_track_info(
    user: AuthenticatedUser,
    query: web::Json<GetTrackInfoParams>,
    app_state: web::Data<AppData>,
) -> HttpResponse {
    let request = query.into_inner();
    let resolved_user = app_state
        .user_resolver
        .resolve_existing_user_and_actor(&user.user_id)
        .await;

    let user_actor = match resolved_user {
        Ok(u) => u.actor,
        Err(e) => return HttpResponse::NotFound().body("User not found"),
    };

    let rez = match user_actor
        .ask(GetTrackMeta {
            track_id: request.track_id,
        })
        .await
    {
        Ok(smth) => HttpResponse::Ok().json(smth),
        Err(e) => return HttpResponse::InternalServerError().body("Could not get track info"),
    };
    rez
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(add_track)
        .service(add_track_multi)
        .service(update_track_info)
        .service(remove_track)
        .service(get_raw)
        .service(get_track_info)
        .service(get_meta)
        .service(get_tracks)
        .service(copy_track);
}
fn bytes_to_f32(bytes: Vec<u8>) -> Vec<f32> {
    use std::convert::TryInto;

    // Each f32 is 4 bytes
    let mut out = Vec::with_capacity(bytes.len() / 4);

    for chunk in bytes.chunks_exact(4) {
        let arr: [u8; 4] = chunk.try_into().unwrap();
        out.push(f32::from_le_bytes(arr)); // little-endian decode
    }

    out
}

async fn next_multipart_field(field: &mut actix_multipart::Field) -> Vec<u8> {
    let mut data = Vec::new();
    while let Some(chunk) = field.next().await {
        data.extend_from_slice(&chunk.unwrap());
    }
    data
}
