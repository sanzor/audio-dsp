use std::{collections::HashMap, sync::Arc};

use crate::app_data::AppData;
use actix_web::{
    delete, get, post, put,
    web::{self, put},
    HttpResponse,
};
use actors::user_actor::{
    create_user_actor_params::CreateUserActorParams, create_user_data::CreateUserData,
    local_players_provider::LocalPlayerProvider, user_actor::UserActor,
};
use domain::{
    actors::messages::{
        player::get_player_state::GetPlayerStateResult,
        user::{get_user_state::GetUserState, remove_user::RemoveUser, update_user::UpdateUser},
    },
    track_meta::TrackMeta,
};
use dsp_core::tracks_provider::LocalTrackStoreProvider;
use kameo::{actor::ActorRef, Actor};
use serde::{Deserialize, Serialize};
use ulid::Ulid;

#[derive(Deserialize, Clone, Debug, Serialize)]
pub struct GetUserDataResult {
    pub players: HashMap<String, GetPlayerStateResult>,
    pub tracks: HashMap<String, TrackMeta>,
}
#[get("/get-user-state/{user_id}")]
async fn get_user_state(path: web::Path<String>, app_state: web::Data<AppData>) -> HttpResponse {
    let user_id = path.into_inner();
    let guard = app_state.user_map.lock().await;
    let user = match guard.get(&user_id).cloned() {
        Some(u) => u,
        None => return HttpResponse::BadRequest().body("Could not find user"),
    };
    let user_data = match user.ask(GetUserState {}).await {
        Ok(data) => data,
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };
    let result = GetUserDataResult {
        players: user_data.players,
        tracks: user_data.tracks,
    };
    HttpResponse::Ok().json(result)
}

#[derive(Deserialize, Clone, Debug, Serialize)]
pub struct CreateUserParams {
    pub user_name: String,
    pub email: String,
}

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct CreateUserResult {
    pub user_id: String,
    pub email: String,
    pub user_name: String,
}

#[post("/create")]
async fn create(path: web::Json<CreateUserParams>, app_state: web::Data<AppData>) -> HttpResponse {
    let request = path.into_inner();
    let id = Ulid::new().to_string();
    let actor_params = CreateUserActorParams {
        player_factory: Arc::clone(&app_state.player_factory),
        user_data: CreateUserData {
            id: id.clone(),
            name: request.user_name.clone(),
            email: request.email.clone(),
        },
        players_provider: Box::new(LocalPlayerProvider::new()),
        tracks_provider: Box::new(LocalTrackStoreProvider::new()),
    };
    let user_actor = UserActor::spawn(UserActor::new(actor_params));
    let mut user_map = app_state.user_map.lock().await;
    user_map.insert(id.clone(), user_actor);
    HttpResponse::Created().json(CreateUserResult {
        user_id: id,
        user_name: request.user_name,
        email: request.email,
    })
}

#[derive(Deserialize)]
pub struct RemoveUserParams {
    pub user_id: String,
}

#[delete("/remove/{user_id}")]
async fn delete(path: web::Path<String>, app_state: web::Data<AppData>) -> HttpResponse {
    let user_id = path.into_inner();
    let user = match get_user_internal(&user_id, &app_state).await {
        Ok(u) => u,
        Err(e) if e.contains("Could not find") => {
            return HttpResponse::NotFound().body("Could not find user")
        }
        _ => return HttpResponse::InternalServerError().body("Could not search user"),
    };
    let result = user.ask(RemoveUser {}).await;
    HttpResponse::NoContent().body("User deleted")
}

#[derive(Deserialize, Debug, Serialize)]
pub struct UpdateUserParams {
    pub user_id: String,
    pub user_name: String,
    pub email: String,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct UserUpdateResult {
    pub user_id: String,
    pub new_email: String,
    pub new_user_name: String,
}
#[put("/update")]
async fn update(path: web::Json<UpdateUserParams>, app_state: web::Data<AppData>) -> HttpResponse {
    let request = path.into_inner();
    let user = match get_user_internal(&request.user_id, &app_state).await {
        Ok(u) => u,
        Err(e) if e.contains("Could not find") => {
            return HttpResponse::NotFound().body("Could not find user")
        }
        _ => return HttpResponse::InternalServerError().body("Could not search user"),
    };

    match user
        .ask(UpdateUser {
            id: request.user_id,
            email: request.email.clone(),
            name: request.user_name,
        })
        .await
    {
        Ok(new_user) => HttpResponse::Ok().json(UserUpdateResult {
            new_email: new_user.new_email,
            new_user_name: new_user.new_name,
            user_id: new_user.id,
        }),
        Err(err) => HttpResponse::InternalServerError().body(format!(
            "Could not update user with cause:{:?}",
            err.to_string()
        )),
    }
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
    cfg.service(create)
        .service(get_user_state)
        .service(delete)
        .service(update);
}
