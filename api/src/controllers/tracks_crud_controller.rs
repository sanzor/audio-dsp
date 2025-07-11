use actix_web::{
    delete, get, post, web::{self}, HttpResponse
};
use actors::user_actor::user_actor::UserActor;
use domain::{
    actors::messages::crud::{
        copy_track::CopyTrack, delete_track::DeleteTrack, get_track::GetRawTrack,
        get_track_info::GetTrackMeta, get_tracks::GetTrackMetas, insert_track::InsertTrack,
        update_track_info::UpdateTrackInfo,
    },
    raw_track::{RawTrack, TrackInfo},
};
use kameo::actor::ActorRef;
use serde::{Deserialize, Serialize};

use crate::app_data::AppData;

#[derive(Deserialize, Serialize)]
pub struct AddTrackParams {
    pub user_id: String,
    pub track: RawTrack,
}

#[derive(Serialize, Deserialize)]
pub struct AddTrackResult {
    pub track_id: String,
    pub user_id: String,
}
#[post("/add-track")]
async fn add_track(path: web::Json<AddTrackParams>, app_state: web::Data<AppData>) -> HttpResponse {
    let request = path.into_inner();

    let user = match get_user_internal(&request.user_id, &app_state).await {
        Ok(u) => u,
        Err(e) => return HttpResponse::NotFound().body("User not found"),
    };

    let rez = match user
        .ask(InsertTrack {
            track: request.track,
        })
        .await
    {
        Ok(smth) => HttpResponse::Ok().json(AddTrackResult {
            track_id: smth.track_id,
            user_id: smth.user_id,
        }),
        Err(e) => return HttpResponse::InternalServerError().body("Could not insert track"),
    };
    rez
}

#[derive(Deserialize)]
pub struct CopyTrackParams {
    pub user_id: String,
    pub track_id: String,
    pub copy_track_name: String,
}

#[post("/copy-track")]
async fn copy_track(
    request_raw: web::Json<CopyTrackParams>,
    app_state: web::Data<AppData>,
) -> HttpResponse {
    let request = request_raw.into_inner();

    let user = match get_user_internal(&request.user_id, &app_state).await {
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
        Err(e) => return HttpResponse::InternalServerError().body("Could not copy track"),
    };
    rez
}

#[derive(Deserialize)]
pub struct UpdateTrackParams {
    user_id: String,
    track_info: TrackInfo,
}
#[post("/update-track-info")]
async fn update_track_info(
    path: web::Json<UpdateTrackParams>,
    app_state: web::Data<AppData>,
) -> HttpResponse {
    let request = path.into_inner();

    let user = match get_user_internal(&request.user_id, &app_state).await {
        Ok(u) => u,
        Err(e) => return HttpResponse::NotFound().body("User not found"),
    };

    let rez = match user
        .ask(UpdateTrackInfo {
            track_info: request.track_info,
        })
        .await
    {
        Ok(smth) => HttpResponse::Ok().json("track added"),
        Err(e) => return HttpResponse::InternalServerError().body("Could not insert track"),
    };
    rez
}
#[derive(Deserialize)]
pub struct RemoveTrackParams {
    pub user_id: String,
    pub track_id: String,
}

#[delete("/remove")]
async fn remove_track(
    path: web::Query<RemoveTrackParams>,
    app_state: web::Data<AppData>,
) -> HttpResponse {
    let request = path.into_inner();

    let user = match get_user_internal(&request.user_id, &app_state).await {
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
    pub user_id: String,
    pub track_id: String,
}
#[get("/get-raw")]
async fn get_raw(
    query: web::Query<GetTrackParams>,
    app_state: web::Data<AppData>,
) -> HttpResponse {
    let request = query.into_inner();

    let user = match get_user_internal(&request.user_id, &app_state).await {
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
    query: web::Query<GetTrackParams>,
    app_state: web::Data<AppData>,
) -> HttpResponse {
    let request = query.into_inner();

    let user = match get_user_internal(&request.user_id, &app_state).await {
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

#[derive(Deserialize)]
pub struct GetAllParams {
    pub user_id: String,
}
#[get("/get-all")]
async fn get_tracks(
    query: web::Query<GetAllParams>,
    app_state: web::Data<AppData>,
) -> HttpResponse {
    let request = query.into_inner();

    let user = match get_user_internal(&request.user_id, &app_state).await {
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
    pub user_id: String,
    pub track_id: String,
}
#[get("/get-track-info")]
async fn get_track_info(
    query: web::Json<GetTrackInfoParams>,
    app_state: web::Data<AppData>,
) -> HttpResponse {
    let request = query.into_inner();
    let guard = app_state.user_map.lock().await;

    let user = match get_user_internal(&request.user_id, &app_state).await {
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
        Err(e) => return HttpResponse::InternalServerError().body("Could not get track info"),
    };
    rez
}

async fn get_user_internal(
    user_id: &str,
    app_state: &AppData,
) -> Result<ActorRef<UserActor>, String> {
    let user_addr = {
        let guard = app_state.user_map.lock().await;
        match guard.get(&user_id.to_string()).cloned() {
            Some(addr) => Ok(addr),
            None => Err("Could not find user".to_string()),
        }
    };
    user_addr
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(add_track)
        .service(update_track_info)
        .service(remove_track)
        .service(get_raw)
        .service(get_track_info)
        .service(get_meta)
        .service(get_tracks)
        .service(copy_track);
}
