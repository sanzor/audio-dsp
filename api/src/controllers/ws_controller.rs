use std::{collections::VecDeque, sync::Arc, time::Duration};

use actix_web::{get, web, HttpRequest, HttpResponse};
use player::audio_source::audio_source::AudioSource;
use serde::Deserialize;
use tokio::{sync::Mutex, time};

use crate::{app_data::AppData, audio::{queue_sink::QueueSink, queue_source::QueueSource}, controllers::utils::get_user_internal};



#[derive(Deserialize)]
pub struct PlayRequest{
    pub user_id:String,
    pub track_id:String
}
#[derive(Deserialize)]
#[serde(tag = "type")]
enum WsMessage {
    Play { track_id: String },
    Stop,
    Pause,
}
#[get("/play")]
async fn play(
    req:HttpRequest,
    stream:web::Payload,
    query:web::Query<PlayRequest>,app_state:web::Data<AppData>)->Result<HttpResponse, actix_web::Error>{
    let user = match get_user_internal(&query.user_id, &app_state).await {
        Ok(u) => u,
        Err(e) => return Ok(HttpResponse::NotFound().body("User not found")),
    };

    let queue=Arc::new(Mutex::new(VecDeque::new()));
    let sink=QueueSink{queue:Arc::clone(&queue)};
    let mut source=QueueSource{queue:queue};
    let (response,mut v,mut  ws_stream)=actix_ws::handle(&req, stream)?;
    
    tokio::spawn(async move{
         let mut interval=tokio::time::interval(Duration::from_millis(10));
         loop{
          tokio::select!{
            _=interval.tick()=>{
                if let Some(frame)=source.next_frame().await{
                    if(ws_stream.send())
                    todo!()
                }
            }
        }
    }
    });
    todo!()
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(play);
       
}