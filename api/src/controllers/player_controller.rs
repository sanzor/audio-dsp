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
async fn play(body: web::Json<PlayRequest>,app_state:web::Data<AppData>)->Result<HttpResponse,actix_web::Error>{
    let player_message=body.into_inner();

    let user=player_message.user_name.ok_or_else(||HttpResponse::BadRequest().body("Invalid user"))?;
    let user_addr={
        let guard=app_state.user_map.lock().await;
            guard
                .get(&user)
                .cloned()
                .ok_or_else(||HttpResponse::NotFound()
                .body("Could not find user"))
    };
    let track_name=player_message.track_name.ok_or_else(||HttpResponse::BadRequest().body("Invalid track name"))?;
    
    let user=user_addr.unwrap().tell(UserPlayerCommand::Play { track_id: Some(track_name)}).await;
    Ok(user)
}


pub fn init(cfg:&mut web::ServiceConfig){
    cfg.service(get_player_state)
       .service(get_user_state);
}