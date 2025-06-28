use actix_web::{post, web::{self, post}, HttpResponse};
use actors::user_actor::user_actor::UserActor;
use domain::{dsp_message::DspMessage, track::Track};
use kameo::actor::ActorRef;
use serde::Deserialize;
use ulid::Ulid;

use crate::app_data::AppData;

#[derive(Deserialize)]
pub struct AddTrackParams{
    user_id:String,
    track:Track
}
#[post("/add-track")]
async fn add_track(path:web::Json<AddTrackParams>,app_state:web::Data<AppData>)->HttpResponse{
    let request=path.into_inner();
    let guard=app_state.user_map.lock().await;
    
    let user=match get_user(&request.user_id, &app_state).await{
        Some(u)=>u,
        None=>return HttpResponse::NotFound().body("User not found")
    };

    user.ask(DspMessage::Insert { user_name: , track_payload: () }).await
    HttpResponse::Created().json("User created")
}

async fn get_user(user_id:&str,app_state:&web::Data<AppData>)->Option<ActorRef<UserActor>>{
    let user={
        let guard=app_state.user_map.lock().await;
        let u=match guard.get(user_id).cloned(){
            Some(us)=>us,
            None=>return None
        };
        u
    };
    Some(user)

}