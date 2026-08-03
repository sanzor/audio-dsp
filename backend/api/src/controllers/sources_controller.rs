use crate::{
    middlewares::role_context::role_context::{ProjectContext, RoleContext},
    tracks::tracks_app_data::TracksAppData,
};
use actix_multipart::Multipart;
use actix_web::{
    delete, get, post,
    web::{self},
    HttpResponse,
};
use domain::{
    raw_track::{RawTrack},
    update_track_info_params::UpdateTrackInfoParams,
};

use serde::{Deserialize, Serialize};
use tracing::{error, info};
use utoipa::{IntoParams, ToSchema};

use domain::{db::TrackId, tracks::track_info::TrackInfo as DomainTrackInfo};

#[derive(Deserialize, Serialize)]
pub struct AddSourceParams {
    pub track: RawTrack,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct AddSourceResult {
    pub track_id: TrackId,
    #[schema(value_type = Object)]
    pub track_info: DomainTrackInfo,
}

#[derive(ToSchema)]
pub struct AddTrackMultipartRequest {
    pub name: String,
    pub extension: String,
    #[schema(value_type = String, format = Binary)]
    pub samples: Vec<u8>,
    pub sample_rate: Option<f32>,
    pub channels: Option<String>,
}

#[utoipa::path(
    post,
    path = "/sources/add-source",
    tag = "sources",
    request_body = serde_json::Value,
    responses((status = 200, description = "Source added", body = serde_json::Value)),
    security(
        ("bearerAuth" = [])
    )
)]
#[post("/add-source")]
pub async fn add_source(
    role: RoleContext,
    project: ProjectContext,
    path: web::Json<serde_json::Value>,
    app_state: web::Data<SourcesAppData>,
) -> HttpResponse {
    if !role.can_edit() {
        return HttpResponse::Forbidden().body("Forbidden");
    }
    let request: AddSourceParams = match serde_json::from_value(path.into_inner()) {
        Ok(r) => r,
        Err(_) => return HttpResponse::BadRequest().body("Invalid payload"),
    };

    match app_state.tracks_service.insert_source(request.track, project.0).await {
        Ok(track) => HttpResponse::Ok().json(AddSourceResult {
            track_id: track.meta.track_id,
            track_info: track.meta.track_info,
        }),
        Err(_e) => HttpResponse::InternalServerError().body("Could not insert track"),
    }
}

#[utoipa::path(
    post,
    path = "/tracks/add-track-multi",
    tag = "tracks",
    request_body(
        content = AddTrackMultipartRequest,
        content_type = "multipart/form-data",
        description = "Upload a WAV file in `samples`, or upload raw float32 PCM bytes in `samples` together with `sample_rate` and `channels` (`Mono` or `Stereo`). MP3 bytes are not decoded by this endpoint."
    ),
    responses((status = 200, description = "Track added (multipart)", body = AddTrackResult)),
    security(
        ("bearerAuth" = [])
    )
)]
#[post("/add-track-multi")]
pub async fn add_track_multi(
    _role: RoleContext,
    project: ProjectContext,
    payload: Multipart,
    app_state: web::Data<TracksAppData>,
) -> HttpResponse {
    info!("add-track-multi request received");
    // if !role.can_edit() {
    //     warn!("add-track-multi rejected: role cannot edit");
    //     return HttpResponse::Forbidden().body("Forbidden");
    // }
    let raw_track = match app_state.multipart_parser.try_parse_multipart(payload).await {
        Ok(r) => r,
        Err(err) => {
            error!(error = %err, "add-track-multi rejected: invalid payload");
            return HttpResponse::BadRequest().body(err);
        }
    };

    match app_state.tracks_service.insert_track(raw_track, project.0).await {
        Ok(track) => {
            info!(track_id = %track.meta.track_id, "add-track-multi insert complete");
            HttpResponse::Ok().json(AddTrackResult {
                track_id: track.meta.track_id,
                track_info: track.meta.track_info,
            })
        }
        Err(err) => {
            error!(error = %err, "add-track-multi insert failed");
            HttpResponse::InternalServerError().body("Could not insert track")
        }
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
    responses((status = 200, description = "Track copied", body = serde_json::Value)),
    security(
        ("bearerAuth" = [])
    )
)]
#[post("/copy-track")]
pub async fn copy_track(
    role: RoleContext,
    request_raw: web::Json<CopyTrackParams>,
    app_state: web::Data<TracksAppData>,
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
    responses((status = 200, description = "Track updated", body = serde_json::Value)),
    security(
        ("bearerAuth" = [])
    )
)]
#[post("/update-track-info")]
pub async fn update_track_info(
    role: RoleContext,
    path: web::Json<UpdateTrackParams>,
    app_state: web::Data<TracksAppData>,
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
    responses((status = 200, description = "Track removed", body = serde_json::Value)),
    security(
        ("bearerAuth" = [])
    )
)]
#[delete("/remove")]
pub async fn remove_track(
    role: RoleContext,
    path: web::Query<RemoveTrackParams>,
    app_state: web::Data<TracksAppData>,
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
    path = "/tracks/get-meta",
    tag = "tracks",
    params(GetTrackParams),
    responses((status = 200, description = "Track metadata", body = serde_json::Value)),
    security(
        ("bearerAuth" = [])
    )
)]
#[get("/get-meta")]
pub async fn get_meta(
    role: RoleContext,
    query: web::Query<GetTrackParams>,
    app_state: web::Data<TracksAppData>,
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
    responses((status = 200, description = "All track metas", body = serde_json::Value)),
    security(
        ("bearerAuth" = [])
    )
)]
#[get("/get-all")]
pub async fn get_tracks(_role: RoleContext, app_state: web::Data<TracksAppData>) -> HttpResponse {
    // if !role.can_view() {
    //     return HttpResponse::Forbidden().body("Forbidden");
    // }
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
    responses((status = 200, description = "Track info", body = serde_json::Value)),
    security(
        ("bearerAuth" = [])
    )
)]
#[get("/get-track-info")]
pub async fn get_track_info(
    role: RoleContext,
    query: web::Json<GetTrackInfoParams>,
    app_state: web::Data<TracksAppData>,
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
        .service(get_track_info)
        .service(get_meta)
        .service(get_tracks)
        .service(copy_track);
}

