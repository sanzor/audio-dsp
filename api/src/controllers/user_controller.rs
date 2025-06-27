use std::{collections::HashMap, hash::Hash, sync::Arc};

use actix_web::{get, post, web::{self, post}, HttpResponse};
use actors::user_actor::user_actor::{UserActor, UserActorParams};
use domain::{actors::{player_state_query_result::PlayerStateQueryResult, user_state_query::UserStateQuery}, track::TrackInfo};
use dsp_core::state::TrackState;
use kameo::spawn;
use serde::{Deserialize, Serialize};
use ulid::Ulid;
use crate::app_data::AppData;

#[derive(Deserialize,Clone,Debug,Serialize)]
pub struct GetUserDataResult{
    players:HashMap<String,PlayerStateQueryResult>,
    tracks:HashMap<String,TrackInfo>
}
#[get("/get-user-state")]
async fn get_user(path:web::Path<String>,app_state:web::Data<AppData>)->HttpResponse{
    let user_id=path.into_inner();
    let guard=app_state.user_map.lock().await;
    let user=match guard.get(&user_id).cloned(){
        Some(u)=>u,
        None=>return HttpResponse::BadRequest().body("Could not find user")
    };
    let user_data=match user.ask(UserStateQuery{}).await{
        Ok(data)=>data,
        Err(e)=>return HttpResponse::InternalServerError().body(e.to_string())
    };
    let result=GetUserDataResult{players:user_data.players,tracks:user_data.tracks};
    HttpResponse::Ok().json(result)
}

#[derive(Deserialize,Clone,Debug,Serialize)]
pub struct InsertUserParams{
    user_name:String
}


#[post("/insert")]
async fn insert(path:web::Json<InsertUserParams>,app_state:web::Data<AppData>)->HttpResponse{
    let request=path.into_inner();
    let guard=app_state.user_map.lock().await;
    let id = Ulid::new().to_string();
    let actor_params=UserActorParams{
        id:id.clone(),
        players:HashMap::new(),track_state:TrackState::new(),processor: Arc::clone(&app_state.processor)};
    let user_actor=spawn(UserActor::new(actor_params));
    let mut user_map=app_state.user_map.lock().await;
    let rez=match user_map.insert(id, user_actor){
        None => return HttpResponse::InternalServerError().body("Could not insert new user"),
        Some(u)=>u
    };
    
    HttpResponse::Created().json("User created")
}