use actix_web::{post, web::{self, post}, HttpResponse};
use actors::user_actor::user_actor::UserActor;
use domain::{actors::crud_command::CrudCommand, dsp_message::DspMessage, track::{Track, TrackInfo}};
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

    let rez=match user.ask(CrudCommand::InsertTrack { track: request.track }).await{
        Ok(smth)=>HttpResponse::Ok().json("track added"),
        Err(e)=>return HttpResponse::InternalServerError().body("Could not insert track")
    };
    rez
}

#[derive(Deserialize)]
pub struct UpdateTrackParams{
    user_id:String,
    track_info:TrackInfo
}
#[post("/update-track")]
async fn add_track(path:web::Json<UpdateTrackParams>,app_state:web::Data<AppData>)->HttpResponse{
    let request=path.into_inner();
    let guard=app_state.user_map.lock().await;
    
    let user=match get_user_internal(&request.user_id, &app_state).await{
        Ok(u)=>u,
        Err(e)=>return HttpResponse::NotFound().body("User not found")
    };

    let rez=match user.ask(CrudCommand::InsertTrack { track: request.track }).await{
        Ok(smth)=>HttpResponse::Ok().json("track added"),
        Err(e)=>return HttpResponse::InternalServerError().body("Could not insert track")
    };
    rez
}
#[derive(Deserialize)]
pub struct RemoveTrackParams{
    user_id:String,
    track_id:String
}

#[post("/remove-track")]
async fn remove_track(path:web::Query<RemoveTrackParams>,app_state:web::Data<AppData>)->HttpResponse{
    let request=path.into_inner();
    let guard=app_state.user_map.lock().await;
    
    let user=match get_user_internal(&request.user_id, &app_state).await{
        Ok(u)=>u,
        Err(e)=>return HttpResponse::NotFound().body("User not found")
    };

    let rez=match user.ask(CrudCommand::RemoveTrack {  track_id:request.track_id }).await{
        Ok(smth)=>HttpResponse::Ok().json("track added"),
        Err(e)=>return HttpResponse::InternalServerError().body("Could not insert track")
    };
    rez
}
#[derive(Deserialize)]
pub struct GetTrackParams{
    pub user_id:String,
    pub track_id:String
}
#[post("/get-track")]
async fn get_track(query:web::Query<GetTrackParams>,app_state:web::Data<AppData>)->HttpResponse{
    let request=query.into_inner();
    let guard=app_state.user_map.lock().await;
    
    let user=match get_user_internal(&request.user_id, &app_state).await{
        Ok(u)=>u,
        Err(e)=>return HttpResponse::NotFound().body("User not found")
    };

    let rez=match user.ask(CrudCommand::GetTrack { track_id:request.track_id}).await{
        Ok(smth)=> HttpResponse::Ok().json(smth),
        Err(e)=>return HttpResponse::InternalServerError().body("Could not insert track")
    };
    rez
}

#[derive(Deserialize)]
pub struct GetTrackInfoParams{
    pub user_id:String,
    pub track_id:String
}
#[post("/get-track-info")]
async fn get_track_info(query:web::Json<GetTrackInfoParams>,app_state:web::Data<AppData>)->HttpResponse{
    let request=query.into_inner();
    let guard=app_state.user_map.lock().await;
    
    let user=match get_user_internal(&request.user_id, &app_state).await{
        Ok(u)=>u,
        Err(e)=>return HttpResponse::NotFound().body("User not found")
    };

    let rez=match user.ask(CrudCommand::GetTrackInfo { track_id: query.track_id}).await{
        Ok(smth)=>HttpResponse::Ok().json(smth),
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