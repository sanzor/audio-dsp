use std::{collections::VecDeque, sync::Arc, time::Duration};

use actix_web::{
    get,
    web::{self},
    HttpRequest, HttpResponse,
};
use async_std::stream::StreamExt;
use domain::db::TrackId;
use player::{
    sink::queue_sink::QueueSink,
    source::{audio_source::AudioSource, queue_source::QueueSource},
    AudioFrame,
};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use utoipa::IntoParams;

use crate::{
    middlewares::jwt::jwt_context::JwtContext,
    player::{player_app_data::PlayerAppData, player_provider::{AttachSinkParams, PlayerProvider}},
};

#[derive(Serialize, Debug, Clone, Deserialize)]
pub enum WebsocketSendMessage {
    AudioFrame { audio_frame: AudioFrame },
}

#[derive(Deserialize, IntoParams)]
pub struct PlayRequest {
    pub track_id: TrackId,
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum WsMessage {
    Play { track_id: TrackId },
    Stop { track_id: TrackId },
    Pause { track_id: TrackId },
    Seek { track_id: TrackId, position: u32 },
}

#[utoipa::path(
    get,
    path = "/ws/run",
    tag = "ws",
    params(PlayRequest),
    responses((status = 101, description = "Websocket upgrade"))
)]
#[get("/run")]
pub async fn run_player(
    jwt: JwtContext,
    req: HttpRequest,
    stream: web::Payload,
    query: web::Query<PlayRequest>,
    app_state: web::Data<PlayerAppData>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = jwt.user_id;
    let track_id = query.track_id.clone();
    let player_service = Arc::clone(&app_state.player_service);

    // Create sink/source pair
    let queue = Arc::new(Mutex::new(VecDeque::new()));
    let sink = QueueSink { queue: Arc::clone(&queue) };
    let mut source = QueueSource { queue };

    // Attach sink to the player (spawns actor if needed)
    let attach_result = player_service
        .attach_sink(AttachSinkParams {
            user_id,
            track_id: track_id.clone(),
            sink: Box::new(sink),
        })
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let sink_id = attach_result.sink_id;

    // Open websocket
    let (response, mut session, mut ws_stream) = actix_ws::handle(&req, stream)?;

    actix_web::rt::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_millis(10));
        loop {
            tokio::select! {
                _ = interval.tick() => {
                    match source.next_frame().await {
                        Some(frame) => {
                            if let Err(e) = send_audio_frame(&mut session, &frame).await {
                                eprintln!("Failed to send audio frame: {e}");
                            }
                        }
                        None => {}
                    }
                },
                msg = ws_stream.next() => {
                    match msg {
                        Some(Ok(actix_ws::Message::Binary(data))) => {
                            if let Ok(text) = std::str::from_utf8(&data) {
                                if let Ok(message) = serde_json::from_str(text) {
                                    if let Err(e) = handle_ws_message(message, user_id, &player_service).await {
                                        eprintln!("Failed to handle message: {e}");
                                    }
                                }
                            }
                        }
                        Some(Ok(actix_ws::Message::Text(data))) => {
                            if let Ok(message) = serde_json::from_str(&data) {
                                if let Err(e) = handle_ws_message(message, user_id, &player_service).await {
                                    eprintln!("Failed to handle message: {e}");
                                }
                            }
                        }
                        Some(Ok(_)) => break,
                        Some(Err(e)) => {
                            eprintln!("Websocket error: {e}");
                            break;
                        }
                        None => {
                            eprintln!("Websocket closed");
                            break;
                        }
                    }
                }
            }
        }

        // Clean up sink on disconnect
        let _ = player_service.remove_sink(user_id, &track_id, &sink_id).await;
    });

    Ok(response)
}

async fn handle_ws_message(
    message: WsMessage,
    user_id: i64,
    player: &Arc<dyn PlayerProvider>,
) -> Result<(), String> {
    match message {
        WsMessage::Play { track_id } => player.play(user_id, &track_id).await,
        WsMessage::Pause { track_id } => player.pause(user_id, &track_id).await,
        WsMessage::Stop { track_id } => player.stop(user_id, &track_id).await,
        WsMessage::Seek { track_id, position } => player.seek(user_id, &track_id, position).await,
    }
}

async fn send_audio_frame(
    session: &mut actix_ws::Session,
    frame: &AudioFrame,
) -> Result<(), actix_web::Error> {
    let payload = serde_json::to_string(&WebsocketSendMessage::AudioFrame {
        audio_frame: frame.clone(),
    })?;
    match session.text(payload).await {
        Err(e) => {
            eprintln!("Could not send frame: {e}");
            Ok(())
        }
        _ => Ok(()),
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(run_player);
}
