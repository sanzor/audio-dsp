use std::{collections::VecDeque, sync::Arc, time::Duration};

use actix_web::{
    get,
    web::{self},
    HttpRequest, HttpResponse,
};
use actors::user_actor::{
    user_actor::UserActor,
    user_attach_sink::{UserAttachSink, UserAttachSinkResult},
};
use async_std::stream::StreamExt;
use domain::actors::messages::user_to_player::{
    user_pause::UserPause, user_play::UserPlay, user_seek::UserSeek, user_stop::UserStop,
};
use kameo::actor::ActorRef;
use player::{
    sink::queue_sink::QueueSink,
    source::{audio_source::AudioSource, queue_source::QueueSource},
    AudioFrame,
};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::{
    app_data::AppData, controllers::utils::get_user_actor_internal,
    dtos::authenticated_user::AuthenticatedUser,
};

#[derive(Serialize, Debug, Clone, Deserialize)]
pub enum WebsocketSendMessage {
    AudioFrame { audio_frame: AudioFrame },
}
#[derive(Deserialize)]
pub struct PlayRequest {
    track_id: String,
}
#[derive(Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum WsMessage {
    Play { track_id: String },
    Stop { track_id: String },
    Pause { track_id: String },
    Seek { track_id: String, position: u32 },
}
#[get("/run")]
async fn run_player(
    user: AuthenticatedUser,
    req: HttpRequest,
    stream: web::Payload,
    query: web::Query<PlayRequest>,
    app_state: web::Data<AppData>,
) -> Result<HttpResponse, actix_web::Error> {
    //fetch user
    let user = match get_user_actor_internal(&user.user_id, &app_state).await {
        Ok(u) => u,
        Err(_e) => return Ok(HttpResponse::NotFound().body("User not found")),
    };

    //create queue
    let queue = Arc::new(Mutex::new(VecDeque::new()));

    //create sink
    let sink = QueueSink {
        queue: Arc::clone(&queue),
    };

    //create source
    let mut source = QueueSource { queue };

    let attach_sink_message = UserAttachSink {
        sink: Box::new(sink),
        track_id: query.track_id.clone(),
    };
    let _: UserAttachSinkResult = match user
        .ask(attach_sink_message)
        .await
        .map_err(|e| e.to_string())
    {
        Err(_e) => {
            return Ok(
                HttpResponse::InternalServerError().body("Could not attach sink to audio player")
            )
        }
        Ok(r) => {
            // println!("Attached sink with result {r:?}");
            r
        }
    };
    //open websocket
    let (response, mut session, mut ws_stream) = actix_ws::handle(&req, stream)?;

    actix::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_millis(10));
        loop {
            tokio::select! {
                _=interval.tick()=>{
                    match source.next_frame().await{
                        Some(frame)=>{

                            if let Err(e)=send_audio_frame(&mut session,&frame).await{
                                eprintln!("Failed to send audio frame {e}")
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
                                         eprintln!("Failed to handle message: {e}");
                                    }
                                }
                            }
                        },
                        Some(Ok(actix_ws::Message::Text(data))) => {
                            if let Ok(message)= serde_json::from_str(&data){
                                handle_ws_message(message, &user).await.unwrap();
                            }
                        }
                        Some(Ok(_))=>{
                            //
                            break;
                        },
                        Some(Err(e))=>{
                            eprintln!("Websocket error {e}")
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

async fn handle_ws_message(
    message: WsMessage,
    user_actor: &ActorRef<UserActor>,
) -> Result<(), String> {
    match message {
        WsMessage::Play { track_id } => {
            user_actor
                .tell(UserPlay { track_id })
                .await
                .map_err(|e| e.to_string())?;
            Ok(())
        }
        WsMessage::Pause { track_id } => {
            user_actor
                .tell(UserPause { track_id })
                .await
                .map_err(|e| e.to_string())?;
            Ok(())
        }
        WsMessage::Stop { track_id } => {
            user_actor
                .tell(UserStop { track_id })
                .await
                .map_err(|e| e.to_string())?;
            Ok(())
        }
        WsMessage::Seek { track_id, position } => {
            user_actor
                .tell(UserSeek { track_id, position })
                .await
                .map_err(|e| e.to_string())?;
            Ok(())
        }
    }
}

async fn send_audio_frame(
    session: &mut actix_ws::Session,
    frame: &AudioFrame,
) -> Result<(), actix_web::Error> {
    let payload = serde_json::to_string(&WebsocketSendMessage::AudioFrame {
        audio_frame: frame.clone(),
    })?;

    session.text(payload).await;
    Ok(())
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(run_player);
}
