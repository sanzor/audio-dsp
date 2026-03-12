use crate::{
    app_data::AppData,
    middlewares::role_context::role_context::RoleContext,
};
use actix_multipart::Multipart;
use actix_web::{
    delete, get, post,
    web::{self},
    HttpResponse,
};
use audiolib::{audio_buffer::AudioBuffer, Channels};
use domain::{
    raw_track::{RawTrack, TrackInfo},
    update_track_info_params::UpdateTrackInfoParams,
};
use futures_util::StreamExt;
use mime_guess::from_ext;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use utoipa::{IntoParams, ToSchema};

use domain::db::TrackId;

#[derive(Deserialize, Serialize)]
pub struct AddTrackParams {
    pub track: RawTrack,
}

#[derive(Serialize, Deserialize)]
pub struct AddTrackResult {
    pub track_id: TrackId,
}

#[utoipa::path(
    post,
    path = "/tracks/add-track",
    tag = "tracks",
    request_body = serde_json::Value,
    responses((status = 200, description = "Track added", body = serde_json::Value))
)]
#[post("/add-track")]
pub async fn add_track(
    role: RoleContext,
    path: web::Json<serde_json::Value>,
    app_state: web::Data<AppData>,
) -> HttpResponse {
    if !role.can_edit() {
        return HttpResponse::Forbidden().body("Forbidden");
    }
    let request: AddTrackParams = match serde_json::from_value(path.into_inner()) {
        Ok(r) => r,
        Err(_) => return HttpResponse::BadRequest().body("Invalid payload"),
    };

    match app_state.tracks_service.insert_track(request.track).await {
        Ok(meta) => HttpResponse::Ok().json(AddTrackResult { track_id: meta.track_id }),
        Err(_e) => HttpResponse::InternalServerError().body("Could not insert track"),
    }
}

#[utoipa::path(
    post,
    path = "/tracks/add-track-multi",
    tag = "tracks",
    request_body(content = String, content_type = "multipart/form-data"),
    responses((status = 200, description = "Track added (multipart)", body = serde_json::Value))
)]
#[post("/add-track-multi")]
pub async fn add_track_multi(
    role: RoleContext,
    mut payload: Multipart,
    app_state: web::Data<AppData>,
) -> HttpResponse {
    if !role.can_edit() {
        return HttpResponse::Forbidden().body("Forbidden");
    }
    let mut name: Option<String> = None;
    let mut extension: Option<String> = None;
    let mut sample_rate: Option<f32> = None;
    let mut channels: Option<Channels> = None;
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
    }
    let samples: Vec<f32> = bytes_to_f32(samples_bytes);
    let length = samples.len() as f32 / sample_rate;
    let raw_track = RawTrack {
        info: TrackInfo { name, extension, length },
        data: AudioBuffer { samples, sample_rate, channels },
    };

    match app_state.tracks_service.insert_track(raw_track).await {
        Ok(meta) => HttpResponse::Ok().json(AddTrackResult { track_id: meta.track_id }),
        Err(_) => HttpResponse::InternalServerError().body("Could not insert track"),
    }
}

#[derive(Deserialize, ToSchema)]
pub struct CopyTrackParams {
    pub track_id: TrackId,
    pub copy_track_name: String,
}

#[utoipa::path(
    post,
    path = "/tracks/copy-track",
    tag = "tracks",
    request_body = CopyTrackParams,
    responses((status = 200, description = "Track copied", body = serde_json::Value))
)]
#[post("/copy-track")]
pub async fn copy_track(
    role: RoleContext,
    request_raw: web::Json<CopyTrackParams>,
    app_state: web::Data<AppData>,
) -> HttpResponse {
    if !role.can_edit() {
        return HttpResponse::Forbidden().body("Forbidden");
    }
    let request = request_raw.into_inner();
    match app_state
        .tracks_service
        .copy_track(&request.track_id, request.copy_track_name)
        .await
    {
        Ok(_) => HttpResponse::Ok().json("track copied"),
        Err(_e) => HttpResponse::InternalServerError().body("Could not copy track"),
    }
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateTrackParams {
    pub track_id: TrackId,
    pub track_name: String,
}

#[utoipa::path(
    post,
    path = "/tracks/update-track-info",
    tag = "tracks",
    request_body = UpdateTrackParams,
    responses((status = 200, description = "Track updated", body = serde_json::Value))
)]
#[post("/update-track-info")]
pub async fn update_track_info(
    role: RoleContext,
    path: web::Json<UpdateTrackParams>,
    app_state: web::Data<AppData>,
) -> HttpResponse {
    if !role.can_edit() {
        return HttpResponse::Forbidden().body("Forbidden");
    }
    let request = path.into_inner();
    match app_state
        .tracks_service
        .update_track_info(
            &request.track_id,
            UpdateTrackInfoParams { track_name: request.track_name },
        )
        .await
    {
        Ok(_) => HttpResponse::Ok().json("track updated"),
        Err(_e) => HttpResponse::InternalServerError().body("Could not update track"),
    }
}

#[derive(Deserialize, IntoParams)]
pub struct RemoveTrackParams {
    pub track_id: TrackId,
}

#[utoipa::path(
    delete,
    path = "/tracks/remove",
    tag = "tracks",
    params(RemoveTrackParams),
    responses((status = 200, description = "Track removed", body = serde_json::Value))
)]
#[delete("/remove")]
pub async fn remove_track(
    role: RoleContext,
    path: web::Query<RemoveTrackParams>,
    app_state: web::Data<AppData>,
) -> HttpResponse {
    if !role.can_edit() {
        return HttpResponse::Forbidden().body("Forbidden");
    }
    let request = path.into_inner();
    match app_state.tracks_service.delete_track(&request.track_id).await {
        Ok(_) => HttpResponse::Ok().json("track removed"),
        Err(_e) => HttpResponse::InternalServerError().body("Could not remove track"),
    }
}

#[derive(Deserialize, IntoParams)]
pub struct GetTrackParams {
    pub track_id: TrackId,
}

#[utoipa::path(
    get,
    path = "/tracks/get-stored-track",
    tag = "tracks",
    params(GetTrackParams),
    responses((status = 200, description = "Track audio bytes"))
)]
#[get("/get-stored-track")]
pub async fn get_stored_track(
    role: RoleContext,
    query: web::Query<GetTrackParams>,
    app_state: web::Data<AppData>,
) -> HttpResponse {
    if !role.can_view() {
        return HttpResponse::Forbidden().body("Forbidden");
    }
    let request = query.into_inner();
    let stored_track = match app_state.tracks_service.get_stored_track(&request.track_id).await {
        Ok(t) => t,
        Err(_e) => return HttpResponse::NotFound().body("Could not find track"),
    };
    let ext = stored_track.track_info.extension.to_lowercase();
    let mime_type = from_ext(&ext).first_or_octet_stream().essence_str().to_owned();
    HttpResponse::Ok()
        .insert_header(("Content-Type", mime_type))
        .insert_header((
            "Content-Disposition",
            format!("inline; filename=\"{}.{}\"", stored_track.track_info.name, ext),
        ))
        .body(stored_track.canonical_audio)
}

#[utoipa::path(
    get,
    path = "/tracks/get-meta",
    tag = "tracks",
    params(GetTrackParams),
    responses((status = 200, description = "Track metadata", body = serde_json::Value))
)]
#[get("/get-meta")]
pub async fn get_meta(
    role: RoleContext,
    query: web::Query<GetTrackParams>,
    app_state: web::Data<AppData>,
) -> HttpResponse {
    if !role.can_view() {
        return HttpResponse::Forbidden().body("Forbidden");
    }
    let request = query.into_inner();
    match app_state.tracks_service.get_track_meta(&request.track_id).await {
        Ok(meta) => HttpResponse::Ok().json(meta),
        Err(_e) => HttpResponse::InternalServerError().body("Could not get track"),
    }
}

#[utoipa::path(
    get,
    path = "/tracks/get-all",
    tag = "tracks",
    responses((status = 200, description = "All track metas", body = serde_json::Value))
)]
#[get("/get-all")]
pub async fn get_tracks(role: RoleContext, app_state: web::Data<AppData>) -> HttpResponse {
    if !role.can_view() {
        return HttpResponse::Forbidden().body("Forbidden");
    }
    match app_state.tracks_service.get_all_track_metas().await {
        Ok(metas) => HttpResponse::Ok().json(metas),
        Err(_e) => HttpResponse::InternalServerError().body("Could not get tracks"),
    }
}

#[derive(Deserialize, ToSchema)]
pub struct GetTrackInfoParams {
    pub track_id: TrackId,
}

#[utoipa::path(
    get,
    path = "/tracks/get-track-info",
    tag = "tracks",
    request_body = GetTrackInfoParams,
    responses((status = 200, description = "Track info", body = serde_json::Value))
)]
#[get("/get-track-info")]
pub async fn get_track_info(
    role: RoleContext,
    query: web::Json<GetTrackInfoParams>,
    app_state: web::Data<AppData>,
) -> HttpResponse {
    if !role.can_view() {
        return HttpResponse::Forbidden().body("Forbidden");
    }
    let request = query.into_inner();
    match app_state.tracks_service.get_track_meta(&request.track_id).await {
        Ok(meta) => HttpResponse::Ok().json(meta),
        Err(_e) => HttpResponse::InternalServerError().body("Could not get track info"),
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(add_track)
        .service(add_track_multi)
        .service(update_track_info)
        .service(remove_track)
        .service(get_stored_track)
        .service(get_track_info)
        .service(get_meta)
        .service(get_tracks)
        .service(copy_track);
}

fn bytes_to_f32(bytes: Vec<u8>) -> Vec<f32> {
    use std::convert::TryInto;
    let mut out = Vec::with_capacity(bytes.len() / 4);
    for chunk in bytes.chunks_exact(4) {
        let arr: [u8; 4] = chunk.try_into().unwrap();
        out.push(f32::from_le_bytes(arr));
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
