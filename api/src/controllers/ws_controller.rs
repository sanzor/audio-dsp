use std::{collections::VecDeque, sync::Arc, time::Duration};

use actix_web::{get, web, HttpRequest, HttpResponse};
use actix_ws::Message;
use player::{audio_source::audio_source::AudioSource, AudioFrame};
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
    let (response,mut v,  ws_stream)=actix_ws::handle(&req, stream)?;
    bytemuck::
    tokio::spawn(async move{
         let mut interval=tokio::time::interval(Duration::from_millis(10));
         loop{
          tokio::select!{
            _=interval.tick()=>{
                if let Some(frame)=source.next_frame().await{
                    let _= v.b(frame).await.unwrap();
                    
                }
            },
            msg=ws_stream.recv()=>{
                match msg{
                    Some(Ok(actix_ws::Message::Text(text)))=>{
                        match serde_json::from_str::<WsMessage>(&text){

                        }
                    }
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

fn send_audio_frame(session: &mut actix_ws::Session, frame: &AudioFrame) {
    let bytes = bytemuck::cast_slice(frame);
    let _ = session.b(Message::Binary(bytes.to_vec()));
}