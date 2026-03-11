use actix_web::{
    get, post,
    web::{self},
    HttpResponse,
};
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};

use crate::app_data::AppData;

#[derive(Debug, Deserialize, IntoParams)]
pub struct GetPlayerStateQuery {
    pub user_id: String,
    pub track_id: String,
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
    query: web::Query<GetPlayerStateQuery>,
    app_state: web::Data<AppData>,
) -> HttpResponse {
    let q = query.into_inner();
    match app_state.player_service.get_state(&q.user_id, &q.track_id).await {
        Ok(state) => HttpResponse::Ok().json(state),
        Err(e) => HttpResponse::NotFound().body(e),
    }
}

#[derive(Deserialize, ToSchema)]
pub struct PlayRequest {
    pub user_id: Option<String>,
    pub track_id: Option<String>,
}
#[utoipa::path(
    post,
    path = "/player/play",
    tag = "player",
    request_body = PlayRequest,
    responses((status = 200, description = "Play"))
)]
#[post("/play")]
pub async fn play(body: web::Json<PlayRequest>, app_state: web::Data<AppData>) -> HttpResponse {
    let req = body.into_inner();
    let user_id = match req.user_id {
        Some(u) => u,
        None => return HttpResponse::BadRequest().body("Missing user_id"),
    };
    let track_id = match req.track_id {
        Some(t) => t,
        None => return HttpResponse::BadRequest().body("Missing track_id"),
    };
    match app_state.player_service.play(&user_id, &track_id).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::NotFound().body(e),
    }
}

#[derive(Deserialize, ToSchema)]
pub struct PauseRequest {
    pub user_id: Option<String>,
    pub track_id: Option<String>,
}
#[utoipa::path(
    post,
    path = "/player/pause",
    tag = "player",
    request_body = PauseRequest,
    responses((status = 200, description = "Pause"))
)]
#[post("/pause")]
pub async fn pause(body: web::Json<PauseRequest>, app_state: web::Data<AppData>) -> HttpResponse {
    let req = body.into_inner();
    let user_id = match req.user_id {
        Some(u) => u,
        None => return HttpResponse::BadRequest().body("Missing user_id"),
    };
    let track_id = match req.track_id {
        Some(t) => t,
        None => return HttpResponse::BadRequest().body("Missing track_id"),
    };
    match app_state.player_service.pause(&user_id, &track_id).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::NotFound().body(e),
    }
}

#[derive(Deserialize, ToSchema)]
pub struct SeekRequest {
    pub user_id: Option<String>,
    pub track_id: Option<String>,
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
pub async fn seek(body: web::Json<SeekRequest>, app_state: web::Data<AppData>) -> HttpResponse {
    let req = body.into_inner();
    let user_id = match req.user_id {
        Some(u) => u,
        None => return HttpResponse::BadRequest().body("Missing user_id"),
    };
    let track_id = match req.track_id {
        Some(t) => t,
        None => return HttpResponse::BadRequest().body("Missing track_id"),
    };
    match app_state.player_service.seek(&user_id, &track_id, req.position).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::NotFound().body(e),
    }
}

#[derive(Deserialize, ToSchema)]
pub struct StopRequest {
    pub user_id: Option<String>,
    pub track_id: Option<String>,
}
#[utoipa::path(
    post,
    path = "/player/stop",
    tag = "player",
    request_body = StopRequest,
    responses((status = 200, description = "Stop"))
)]
#[post("/stop")]
pub async fn stop(body: web::Json<StopRequest>, app_state: web::Data<AppData>) -> HttpResponse {
    let req = body.into_inner();
    let user_id = match req.user_id {
        Some(u) => u,
        None => return HttpResponse::BadRequest().body("Missing user_id"),
    };
    let track_id = match req.track_id {
        Some(t) => t,
        None => return HttpResponse::BadRequest().body("Missing track_id"),
    };
    match app_state.player_service.stop(&user_id, &track_id).await {
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
