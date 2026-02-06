use actix_web::{
    get, post,
    web::{self},
    HttpResponse,
};
use domain::actors::messages::user_to_player::{
    user_pause::UserPause, user_play::UserPlay, user_seek::UserSeek, user_stop::UserStop,
};
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};

use crate::app_data::AppData;

#[derive(Debug, Deserialize, IntoParams)]
pub struct GetPlayerState {
    pub user_id: String,
    pub player_id: String,
}
#[utoipa::path(
    get,
    path = "/player/get-player-state",
    tag = "player",
    params(GetPlayerState),
    responses((status = 200, description = "Player state (placeholder)", body = String))
)]
#[get("/get-player-state")]
pub async fn get_player_state(path: web::Query<GetPlayerState>) -> String {
    let _get_player_state = path.into_inner();
    "Hello".to_owned()
}

#[derive(Deserialize, ToSchema)]
pub struct PlayRequest {
    pub user_id: Option<String>,
    pub track_name: Option<String>,
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
    let play_message = body.into_inner();

    let user_id = match play_message.user_id {
        None => return HttpResponse::BadRequest().body("Invalid user"),
        Some(u) => u,
    };
    let actor = match app_state
        .user_resolver
        .resolve_existing_user_and_actor(&user_id)
        .await
    {
        Ok(addr) => addr.actor,
        Err(_e) => return HttpResponse::NotFound().body("Could not find user"),
    };

    let track_name = match play_message.track_name {
        Some(track) => track,
        None => return HttpResponse::BadRequest().body("Invalid track name"),
    };

    let _ = actor
        .tell(UserPlay {
            track_id: track_name,
        })
        .await;
    HttpResponse::Ok().finish()
}

#[derive(Deserialize, ToSchema)]
pub struct PauseRequest {
    pub user_id: Option<String>,
    pub track_name: Option<String>,
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
    let pause_message = body.into_inner();

    let user_id = match pause_message.user_id {
        None => return HttpResponse::BadRequest().body("Invalid user"),
        Some(u) => u,
    };

    let actor = match app_state
        .user_resolver
        .resolve_existing_user_and_actor(&user_id)
        .await
    {
        Ok(addr) => addr.actor,
        Err(_e) => return HttpResponse::NotFound().body("Could not find user"),
    };

    let track_name = match pause_message.track_name {
        Some(track) => track,
        None => return HttpResponse::BadRequest().body("Invalid track name"),
    };

    let _ = actor
        .tell(UserPause {
            track_id: track_name,
        })
        .await;
    HttpResponse::Ok().finish()
}

#[derive(Deserialize, ToSchema)]
pub struct SeekRequest {
    pub user_id: Option<String>,
    pub track_name: Option<String>,
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
    let seek_message = body.into_inner();

    let user_id = match seek_message.user_id {
        None => return HttpResponse::BadRequest().body("Invalid user"),
        Some(u) => u,
    };
    let actor = match app_state
        .user_resolver
        .resolve_existing_user_and_actor(&user_id)
        .await
    {
        Ok(addr) => addr.actor,
        Err(_e) => return HttpResponse::NotFound().body("Could not find user"),
    };

    let track_name = match seek_message.track_name {
        Some(track) => track,
        None => return HttpResponse::BadRequest().body("Invalid track name"),
    };

    let _ = actor
        .tell(UserSeek {
            track_id: track_name,
            position: seek_message.position,
        })
        .await;
    HttpResponse::Ok().finish()
}

#[derive(Deserialize, ToSchema)]
pub struct StopRequest {
    pub user_id: Option<String>,
    pub track_name: Option<String>,
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
    let player_message = body.into_inner();

    let user_id = match player_message.user_id {
        None => return HttpResponse::BadRequest().body("Invalid user"),
        Some(u) => u,
    };
    let actor = match app_state
        .user_resolver
        .resolve_existing_user_and_actor(&user_id)
        .await
    {
        Ok(addr) => addr.actor,
        Err(_e) => return HttpResponse::NotFound().body("Could not find user"),
    };

    let track_name = match player_message.track_name {
        Some(track) => track,
        None => return HttpResponse::BadRequest().body("Invalid track name"),
    };

    let _ = actor
        .tell(UserStop {
            track_id: track_name,
        })
        .await;
    HttpResponse::Ok().finish()
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_player_state)
        .service(play)
        .service(pause)
        .service(seek)
        .service(stop);
}
