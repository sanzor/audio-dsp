use std::{collections::HashMap, sync::Arc};

use actix_web::{delete, get, post, web::{self, get, post}, HttpResponse};
use actors::user_actor::user_actor::{UserActor, CreateUserActorParams, CreateUserData};
use domain::{actors::{player_state_query_result::PlayerStateQueryResult, user_command::UserCommand, user_state_query::UserStateQuery, user_update_params::UserUpdateParams}, track::TrackInfo};
use dsp_core::state::Tracks;
use kameo::spawn;
use serde::{Deserialize, Serialize};
use ulid::Ulid;
use crate::{app_data::AppData};

#[derive(Deserialize,Clone,Debug,Serialize)]
pub struct GetUserDataResult{
    players:HashMap<String,PlayerStateQueryResult>,
    tracks:HashMap<String,TrackInfo>
}
#[get("/get-user")]
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
    user_name:String,
    email:String
}

#[derive(Serialize,Clone,Debug)]
pub struct InsertUserResult{
    user_id:String,
    email:String,
    user_name:String,
}

#[post("/create")]
async fn create(path:web::Json<InsertUserParams>,app_state:web::Data<AppData>)->InsertUserResult{
    let request=path.into_inner();
    let guard=app_state.user_map.lock().await;
    let id = Ulid::new().to_string();
    let actor_params=CreateUserActorParams{
        user_data:CreateUserData{
            id:id,
            name:request.user_name,
            email:request.email
        },
        players:HashMap::new(),
        tracks:Tracks::new(),
        processor: Arc::clone(&app_state.processor)
    };
    let user_actor=spawn(UserActor::new(actor_params));
    let mut user_map=app_state.user_map.lock().await;
    let rez=match user_map.insert(id, user_actor){
        None => return HttpResponse::InternalServerError().body("Could not insert new user"),
        Some(u)=>u
    };
    HttpResponse::Created().json(InsertUserResult{user_id:id,user_name:request.user_name,email:request.email})
}



#[delete("/remove")]
async fn delete(path:web::Path<String>,app_state:web::Data<AppData>)->HttpResponse{
    let user_id=path.into_inner();
    let guard=app_state.user_map.lock().await;

    let actor_ref={
        let mut user_map=app_state.user_map.lock().await;
        match user_map.get(&user_id){
            None => return HttpResponse::InternalServerError().body("Could not insert new user"),
            Some(u)=>u
        }
    };
    let result=actor_ref.ask(UserCommand::Remove).await;
    HttpResponse::NoContent().body("User deleted")
}

#[derive(Deserialize,Debug)]
pub struct UpdateUserParams{
    user_id:String,
    user_name:String,
    email:String
}

#[derive(Deserialize,Debug)]
pub struct UserUpdateResult{
    user_id:String,
    email:String,
    user_name:String
}
#[post("/update")]
async fn update(path:web::Json<UpdateUserParams>,app_state:web::Data<AppData>)->HttpResponse{
    let request=path.into_inner();
    let guard=app_state.user_map.lock().await;
    let user_addr={
        let guard=app_state.user_map.lock().await;
        match guard.get(&user).cloned(){
            Some(addr)=>addr,
            None=>return HttpResponse::NotFound().body("Could not find user")
        }
    };

    let result=user_addr.ask(
        UserCommand::Update(UserUpdateParams{id:request.user_id,email:request.email,name:request.email})).await;
    HttpResponse::Ok().json("User created")
}

async fn get_user(user_id:&str,app_state:&AppData)->{
     let user_addr={
        let guard=app_state.user_map.lock().await;
        match guard.get(&user_id).cloned(){
            Some(addr)=>addr,
            None=>return HttpResponse::NotFound().body("Could not find user")
        }

    };
}
pub fn init(cfg:&mut web::ServiceConfig){
    cfg
       .service(create)
       .service(get_user);
}