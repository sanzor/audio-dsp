use actix_web::{post, web::{self, post}, HttpResponse};
use actors::user_actor::user_actor::UserActor;
use domain::{actors::user_crud_command::UserCrudCommand, dsp_message::DspMessage, track::Track};
use kameo::actor::ActorRef;
use serde::Deserialize;

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
    
    let user=match get_user_internal(&request.user_id, &app_state).await{
        Ok(u)=>u,
        Err(e)=>return HttpResponse::NotFound().body("User not found")
    };

    let rez=match user.ask(UserCrudCommand::InsertTrack { user_id: Some(request.user_id), track: request.track }).await{
        Ok(smth)=>HttpResponse::Ok().json("track added"),
        Err(e)=>return HttpResponse::InternalServerError().body("Could not insert track")
    };
    rez
}

#[post("/remove-track")]
async fn remove_track(path:web::Json<AddTrackParams>,app_state:web::Data<AppData>)->HttpResponse{
    let request=path.into_inner();
    let guard=app_state.user_map.lock().await;
    
    let user=match get_user_internal(&request.user_id, &app_state).await{
        Ok(u)=>u,
        Err(e)=>return HttpResponse::NotFound().body("User not found")
    };

    let rez=match user.ask(UserCrudCommand::InsertTrack { user_id: Some(request.user_id), track: request.track }).await{
        Ok(smth)=>HttpResponse::Ok().json("track added"),
        Err(e)=>return HttpResponse::InternalServerError().body("Could not insert track")
    };
    rez
}

#[post("/get-track")]
async fn get_track(path:web::Json<AddTrackParams>,app_state:web::Data<AppData>)->HttpResponse{
    let request=path.into_inner();
    let guard=app_state.user_map.lock().await;
    
    let user=match get_user_internal(&request.user_id, &app_state).await{
        Ok(u)=>u,
        Err(e)=>return HttpResponse::NotFound().body("User not found")
    };

    let rez=match user.ask(UserCrudCommand::InsertTrack { user_id: Some(request.user_id), track: request.track }).await{
        Ok(smth)=>HttpResponse::Ok().json("track added"),
        Err(e)=>return HttpResponse::InternalServerError().body("Could not insert track")
    };
    rez
}

#[post("/get-track-info")]
async fn get_track_info(path:web::Json<AddTrackParams>,app_state:web::Data<AppData>)->HttpResponse{
    let request=path.into_inner();
    let guard=app_state.user_map.lock().await;
    
    let user=match get_user_internal(&request.user_id, &app_state).await{
        Ok(u)=>u,
        Err(e)=>return HttpResponse::NotFound().body("User not found")
    };

    let rez=match user.ask(UserCrudCommand::InsertTrack { user_id: Some(request.user_id), track: request.track }).await{
        Ok(smth)=>HttpResponse::Ok().json("track added"),
        Err(e)=>return HttpResponse::InternalServerError().body("Could not insert track")
    };
    rez
}



async fn get_user_internal(user_id:&str,app_state:&AppData)->Result<ActorRef<UserActor>,String>{
     let user_addr={
        let guard=app_state.user_map.lock().await;
        match guard.get(&user_id.to_string()).cloned(){
            Some(addr)=>Ok(addr),
            None=>Err("Could not find user".to_string())
        }

    };
    user_addr
}