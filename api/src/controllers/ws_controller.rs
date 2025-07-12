use std::{collections::VecDeque, sync::Arc};

use actix_web::{get, web, HttpRequest, HttpResponse};
use serde::Deserialize;
use tokio::sync::Mutex;

use crate::{app_data::AppData, audio::{queue_sink::QueueSink, queue_source::QueueSource}, controllers::utils::get_user_internal};



#[derive(Deserialize)]
pub struct PlayRequest{
    pub user_id:String,
    pub track_id:String
}

#[get("/play")]
async fn play(
    req:HttpRequest,
    stream:web::Payload,
    query:web::Query<PlayRequest>,app_state:web::Data<AppData>)->HttpResponse{
    let user = match get_user_internal(&query.user_id, &app_state).await {
        Ok(u) => u,
        Err(e) => return HttpResponse::NotFound().body("User not found"),
    };
    let queue=Arc::new(Mutex::new(VecDeque::new()));
    let sink=QueueSink{queue:Arc::clone(&queue)};
    let source=QueueSource{queue:queue};
    tokio::sp
    todo!()
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(play);
       
}