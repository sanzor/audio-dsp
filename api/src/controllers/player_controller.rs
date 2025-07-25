use actix_web::{
    get, post,
    web::{self},
    HttpResponse,
};
use domain::actors::messages::user_to_player::{
    user_pause::UserPause, user_play::UserPlay, user_seek::UserSeek, user_stop::UserStop,
};
use serde::Deserialize;

use crate::app_data::AppData;

#[derive(Debug, Deserialize)]
pub struct GetPlayerState {
    pub user_id: String,
    pub player_id: String,
}
#[get("/get-player-state")]
async fn get_player_state(path: web::Query<GetPlayerState>) -> String {
    let get_player_state = path.into_inner();
    format!("Hello")
}

#[derive(Deserialize)]
pub struct PlayRequest {
    pub user_name: Option<String>,
    pub track_name: Option<String>,
}
#[post("/play")]
async fn play(body: web::Json<PlayRequest>, app_state: web::Data<AppData>) -> HttpResponse {
    let play_message = body.into_inner();

    let user = match play_message.user_name {
        None => return HttpResponse::BadRequest().body("Invalid user"),
        Some(u) => u,
    };
    let user_addr = {
        let guard = app_state.user_map.lock().await;
        match guard.get(&user).cloned() {
            Some(addr) => addr,
            None => return HttpResponse::NotFound().body("Could not find user"),
        }
    };

    let track_name = match play_message.track_name {
        Some(track) => track,
        None => return HttpResponse::BadRequest().body("Invalid track name"),
    };

    let _ = user_addr
        .tell(UserPlay {
            track_id: track_name,
        })
        .await;
    HttpResponse::Ok().finish()
}

#[derive(Deserialize)]
pub struct PauseRequest {
    pub user_name: Option<String>,
    pub track_name: Option<String>,
}
#[post("/pause")]
async fn pause(body: web::Json<PauseRequest>, app_state: web::Data<AppData>) -> HttpResponse {
    let pause_message = body.into_inner();

    let user = match pause_message.user_name {
        None => return HttpResponse::BadRequest().body("Invalid user"),
        Some(u) => u,
    };
    let user_addr = {
        let guard = app_state.user_map.lock().await;
        match guard.get(&user).cloned() {
            Some(addr) => addr,
            None => return HttpResponse::NotFound().body("Could not find user"),
        }
    };

    let track_name = match pause_message.track_name {
        Some(track) => track,
        None => return HttpResponse::BadRequest().body("Invalid track name"),
    };

    let _ = user_addr
        .tell(UserPause {
            track_id: track_name,
        })
        .await;
    HttpResponse::Ok().finish()
}

#[derive(Deserialize)]
pub struct SeekRequest {
    pub user_name: Option<String>,
    pub track_name: Option<String>,
    pub position: u32,
}
#[post("/seek")]
async fn seek(body: web::Json<SeekRequest>, app_state: web::Data<AppData>) -> HttpResponse {
    let seek_message = body.into_inner();

    let user = match seek_message.user_name {
        None => return HttpResponse::BadRequest().body("Invalid user"),
        Some(u) => u,
    };
    let user_addr = {
        let guard = app_state.user_resolver.resolve_user(google_user_info, build_params).lock().await;
        match guard.get(&user).cloned() {
            Some(addr) => addr,
            None => return HttpResponse::NotFound().body("Could not find user"),
        }
    };

    let track_name = match seek_message.track_name {
        Some(track) => track,
        None => return HttpResponse::BadRequest().body("Invalid track name"),
    };

    let _ = user_addr
        .tell(UserSeek {
            track_id: track_name,
            position: seek_message.position,
        })
        .await;
    HttpResponse::Ok().finish()
}

#[post("/stop")]
async fn stop(body: web::Json<PauseRequest>, app_state: web::Data<AppData>) -> HttpResponse {
    let player_message = body.into_inner();

    let user = match player_message.user_name {
        None => return HttpResponse::BadRequest().body("Invalid user"),
        Some(u) => u,
    };
    let user_addr = {
        let guard = app_state.user_map.lock().await;
        match guard.get(&user).cloned() {
            Some(addr) => addr,
            None => return HttpResponse::NotFound().body("Could not find user"),
        }
    };

    let track_name = match player_message.track_name {
        Some(track) => track,
        None => return HttpResponse::BadRequest().body("Invalid track name"),
    };

    let _ = user_addr
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
