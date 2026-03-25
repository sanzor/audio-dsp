use actix_web::{
    get, post,
    web::{self},
    HttpResponse,
};
use domain::db::TrackId;
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};

use crate::{
    middlewares::{
        jwt::jwt_context::JwtContext,
        role_context::role_context::RoleContext,
    },
    player::player_app_data::PlayerAppData,
};

#[derive(Debug, Deserialize, IntoParams)]
pub struct GetPlayerStateQuery {
    pub track_id: TrackId,
}

#[utoipa::path(
    get,
    path = "/player/get-player-state",
    tag = "player",
    params(GetPlayerStateQuery),
    responses((status = 200, description = "Player state", body = String))
)]
#[get("/get-player-state")]
pub async fn get_player_state(
    jwt: JwtContext,
    role: RoleContext,
    query: web::Query<GetPlayerStateQuery>,
    app_state: web::Data<PlayerAppData>,
) -> HttpResponse {
    if !role.can_view() {
        return HttpResponse::Forbidden().body("Forbidden");
    }
    let q = query.into_inner();
    match app_state.player_service.get_state(jwt.user_id, q.track_id).await {
        Ok(state) => HttpResponse::Ok().json(state),
        Err(e) => HttpResponse::NotFound().body(e),
    }
}

#[derive(Deserialize, ToSchema)]
pub struct PlayRequest {
    pub track_id: TrackId,
}

#[utoipa::path(
    post,
    path = "/player/play",
    tag = "player",
    request_body = PlayRequest,
    responses((status = 200, description = "Play"))
)]
#[post("/play")]
pub async fn play(
    jwt: JwtContext,
    role: RoleContext,
    body: web::Json<PlayRequest>,
    app_state: web::Data<PlayerAppData>,
) -> HttpResponse {
    if !role.can_edit() {
        return HttpResponse::Forbidden().body("Forbidden");
    }
    let req = body.into_inner();
    match app_state.player_service.play(jwt.user_id, req.track_id).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::NotFound().body(e),
    }
}

#[derive(Deserialize, ToSchema)]
pub struct PauseRequest {
    pub track_id: TrackId,
}

#[utoipa::path(
    post,
    path = "/player/pause",
    tag = "player",
    request_body = PauseRequest,
    responses((status = 200, description = "Pause"))
)]
#[post("/pause")]
pub async fn pause(
    jwt: JwtContext,
    role: RoleContext,
    body: web::Json<PauseRequest>,
    app_state: web::Data<PlayerAppData>,
) -> HttpResponse {
    if !role.can_edit() {
        return HttpResponse::Forbidden().body("Forbidden");
    }
    let req = body.into_inner();
    match app_state.player_service.pause(jwt.user_id, req.track_id).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::NotFound().body(e),
    }
}

#[derive(Deserialize, ToSchema)]
pub struct SeekRequest {
    pub track_id: TrackId,
    pub position: u32,
}

#[utoipa::path(
    post,
    path = "/player/seek",
    tag = "player",
    request_body = SeekRequest,
    responses((status = 200, description = "Seek"))
)]
#[post("/seek")]
pub async fn seek(
    jwt: JwtContext,
    role: RoleContext,
    body: web::Json<SeekRequest>,
    app_state: web::Data<PlayerAppData>,
) -> HttpResponse {
    if !role.can_edit() {
        return HttpResponse::Forbidden().body("Forbidden");
    }
    let req = body.into_inner();
    match app_state.player_service.seek(jwt.user_id, req.track_id, req.position).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::NotFound().body(e),
    }
}

#[derive(Deserialize, ToSchema)]
pub struct StopRequest {
    pub track_id: TrackId,
}

#[utoipa::path(
    post,
    path = "/player/stop",
    tag = "player",
    request_body = StopRequest,
    responses((status = 200, description = "Stop"))
)]
#[post("/stop")]
pub async fn stop(
    jwt: JwtContext,
    role: RoleContext,
    body: web::Json<StopRequest>,
    app_state: web::Data<PlayerAppData>,
) -> HttpResponse {
    if !role.can_edit() {
        return HttpResponse::Forbidden().body("Forbidden");
    }
    let req = body.into_inner();
    match app_state.player_service.stop(jwt.user_id, req.track_id).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::NotFound().body(e),
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_player_state)
        .service(play)
        .service(pause)
        .service(seek)
        .service(stop);
}
