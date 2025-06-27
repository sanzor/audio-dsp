use std::collections::HashMap;

use actix_web::{get, web, HttpResponse};
use domain::{actors::{player_state_query_result::PlayerStateQueryResult, user_state_query::UserStateQuery}, track::TrackInfo};
use serde::{Deserialize, Serialize};

use crate::actor_registry::AppData;

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