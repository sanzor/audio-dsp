use actix_web::{post, web::{self, post}, HttpResponse};
use serde::Deserialize;
use ulid::Ulid;

use crate::app_data::AppData;

#[derive(Deserialize)]
pub struct AddTrackParams{

}
#[post("/add-track")]
async fn add_track(path:web::Json<AddTrackParams>,app_state:web::Data<AppData>)->HttpResponse{
    let request=path.into_inner();
    let guard=app_state.user_map.lock().await;
    let id = Ulid::new().to_string();
    let mut user_map=app_state.user_map.lock().await;
    let rez=match user_map.insert(id, user_actor){
        None => return HttpResponse::InternalServerError().body("Could not insert new user"),
        Some(u)=>u
    };
    HttpResponse::Created().json("User created")
}