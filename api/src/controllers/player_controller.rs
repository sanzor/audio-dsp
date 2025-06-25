use actix_web::{get, post, web::{self}, HttpResponse, Responder,Result};
use domain::actors::user_player_command::UserPlayerCommand;
use serde::Deserialize;

use crate::actor_registry::AppData;


#[derive(Debug,Deserialize)]
pub struct GetPlayerState{
    pub user_id:String,
    pub player_id:String
}
#[get("/get-player-state")]
async fn get_player_state (path:web::Query<GetPlayerState>)->String{
    let get_player_state=path.into_inner();
    format!("Hello")
}

#[get("/get-user-state/{id}")]
async fn get_user_state(path:web::Path<String>)->String{
    let user_id=path.into_inner();
    "".to_string()
}


#[derive(Deserialize)]
pub struct PlayRequest{
    pub user_name:Option<String>,
    pub track_name:Option<String>
}
#[post("/play")]
async fn play(body: web::Json<PlayRequest>,app_state:web::Data<AppData>)->Result<HttpResponse>{
    let player_message=body.into_inner();
    if(player_message.user_name.is_none()){
        return Ok(HttpResponse::BadRequest().body("Invalid user"))
    }
    let user=player_message.user_name.unwrap();
    let user_ref=app_state.user_map.lock().await.get(&user);
    if(user_ref.is_none()){
        return Ok(HttpResponse::NotFound().body("Could not find user"))
    }
    if(body.track_name.is_none()){
        return Ok(HttpResponse::BadRequest().body("Invalid track name"))
    }
    let track_name=body.track_name.unwrap();
    
    let user=user_ref.unwrap().tell(UserPlayerCommand::Play { track_id: Some(track_name)}).await;
    todo!()
}


pub fn init(cfg:&mut web::ServiceConfig){
    cfg.service(get_player_state)
       .service(get_user_state);
}