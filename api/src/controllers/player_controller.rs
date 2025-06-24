use actix_web::{get,post,web::{self, Query},HttpResponse,Responder};
use serde::Deserialize;


#[derive(Debug,Deserialize)]
pub struct GetPlayerState{
    pub user_id:String,
    pub player_id:String
}
#[get("/get-player-state")]
async fn get_player_state (path:web::Query<GetPlayerState>)->String{
    let id=path.into_inner();
    format!("Hello")
}

#[get("/get-user-state/{id}")]
async fn get_user_state(path:web::Path<String>)->String{
    let user_id=path.into_inner();
    
}