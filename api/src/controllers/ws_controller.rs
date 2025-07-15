use std::{collections::VecDeque, sync::Arc, time::Duration};

use actix_web::{get, web, HttpRequest, HttpResponse};
use actors::user_actor::user_actor::UserActor;
use async_std::stream::StreamExt;
use domain::actors::messages::user_to_player::{user_pause::UserPause, user_play::UserPlay, user_seek::UserSeek, user_stop::UserStop};
use kameo::actor::ActorRef;
use player::{audio_source::audio_source::AudioSource, AudioFrame};
use serde::Deserialize;
use tokio::sync::Mutex;

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
    Stop {track_id:String},
    Pause {track_id:String},
    Seek{track_id:String,position:u32}
}
#[get("/run")]
async fn run_player(
    req:HttpRequest,
    stream:web::Payload,
    query:web::Query<PlayRequest>,app_state:web::Data<AppData>)->Result<HttpResponse, actix_web::Error>{

    //fetch user
    let user = match get_user_internal(&query.user_id, &app_state).await {
        Ok(u) => u,
        Err(e) => return Ok(HttpResponse::NotFound().body("User not found")),
    };

    ///create queue
    let queue=Arc::new(Mutex::new(VecDeque::new()));


    ///create sink
    let sink=QueueSink{queue:Arc::clone(&queue)};

    ///create source
    let mut source=QueueSource{queue:queue};

    //open websocket
    let (response, mut session,mut  ws_stream)=actix_ws::handle(&req, stream)?;

    actix::spawn(async move{
         let mut interval=tokio::time::interval(Duration::from_millis(10));
         loop{
          tokio::select!{
            _=interval.tick()=>{
                match source.next_frame().await{
                    Some(frame)=>{
                        if let Err(e)=send_audio_frame(&mut session,&frame).await{
                            eprintln!("Failed to send audio frame {:?}",e)
                        }
                    },
                    None=>{
                        eprintln!("No frame available")
                    }
                }
            },
            msg=ws_stream.next()=>{
                match msg{
                    Some(Ok(actix_ws::Message::Binary(data)))=>{
                        if let Ok(text)= std::str::from_utf8(&data){
                             if let Ok(message)= serde_json::from_str(text){
                                if let Err(e) = handle_ws_message(message, &user).await {
                                     eprintln!("Failed to handle message: {}", e);
                                }
                            }
                        }
                    },
                    Some(Ok(actix_ws::Message::Text(data))) => {
                        let message: WsMessage = serde_json::from_str(&data).unwrap();
                        let _=handle_ws_message(message, &user).await.unwrap();
                    }
                    Some(Ok(_))=>{
                        //
                        break;
                    },
                    Some(Err(e))=>{
                        eprintln!("Websocket error {:?}",e)
                    },
                    None=>{
                        eprintln!("Websocket closed");
                        break;
                    }
                }
            }
            
        }
    }
    });
    Ok(response)
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(run_player);
     
}

async fn handle_ws_message(message:WsMessage,user_actor:&ActorRef<UserActor>)->Result<(),String>{
    match message{
        WsMessage::Play { track_id }=>{
            let res=user_actor.tell(UserPlay{track_id}).await.map_err(|e|e.to_string())?;
            Ok(())
        },
        WsMessage::Pause { track_id }=>{
             let res=user_actor.tell(UserPause{track_id:track_id}).await.map_err(|e|e.to_string())?;
             Ok(())
        },
        WsMessage::Stop { track_id }=>{
             let res=user_actor.tell(UserStop{track_id:track_id}).await.map_err(|e|e.to_string())?;
             Ok(())
        }
         WsMessage::Seek { track_id ,position}=>{
             let res=user_actor.tell(UserSeek{track_id:track_id,position:position}).await.map_err(|e|e.to_string())?;
             Ok(())
        }
    }
}
async fn send_audio_frame(session: &mut actix_ws::Session, frame: &AudioFrame)->Result<(),actix_web::Error> {
    let bytes = bytemuck::cast_slice::<f32,u8>(frame);
    let _ = session.binary(bytes.to_vec()).await?;
    Ok(())
}

fn decode_audio_frame(bytes:&[u8])->Option<AudioFrame>{
    if bytes.len() % std::mem::size_of::<f32>()!=0{
        return None
    }
    bytemuck::try_cast_slice::<u8,f32>(bytes).ok().map(|sl|sl.to_vec())
}