
use core::panic;
use std::{collections::HashMap, sync::Arc};

use actix_http::{ StatusCode};
use actix_web::{test, web, App};

use actors::user_actor::player_factory::PlayerFactory;
use audiolib::Channels;
use domain::actors::messages::user_to_player::user_play::UserPlay;
use futures_util::{stream::SplitStream, SinkExt, StreamExt};
use rstest::rstest;
use serde::{de::DeserializeOwned, Deserialize};
use tokio::{net::TcpStream, sync::Mutex};
use tokio_tungstenite::{connect_async, tungstenite::{Message, Utf8Bytes}, MaybeTlsStream, WebSocketStream};
use ulid::Ulid;

use crate::{
    app_data::AppData,
    controllers::{tracks_crud_controller::{self, AddTrackParams}, ws_controller::{self, WebsocketSendMessage}},
    player_controller_test::utils::{create_user_actor, insert_track, make_raw_track_from_samples}
};


#[rstest]
#[actix_web::test]
async fn can_start_player_ws() -> Result<(), String> {
    let track = make_raw_track_from_samples(vec![1_f32,2_f32], Channels::Mono);
    let id = Ulid::new();
    let user = create_user_actor(id);
    let mut user_map = HashMap::new();
    user_map.insert(id.to_string(), user);
    let app_data = AppData {
        player_factory: Arc::new(PlayerFactory {}),
        user_map: Arc::new(Mutex::new(user_map)),
    };
    let mut app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_data))
            .service(web::scope("/ws").configure(ws_controller::init))
            .service(web::scope("/tracks").configure(tracks_crud_controller::init)),
    )
    .await;

    let insert_result = insert_track(
        &mut app,
        AddTrackParams {
            track: track,
            user_id: id.to_string(),
        },
    ).await?;
    
    let url = format!(
        "ws://127.0.0.1:8080/ws/run?user_id={}&track_id={}",
        id, insert_result.track_id
    );
    
    let (mut ws_stream, _) = connect_async(&url).await.expect("Failed to connect");
    let(mut write,mut ws_reader)=ws_stream.split();
   
    let play_request=   serde_json::to_string(&UserPlay{track_id:insert_result.track_id}).unwrap();
    
    let v=write.send(Message::Text(play_request.into())).await;
    let msg=read::<WebsocketSendMessage>(&mut ws_reader).await?;
    let frame=match msg {
        WebsocketSendMessage::AudioFrame { audio_frame  }=>audio_frame,
        _ =>panic!()
    };



    
   
    todo!();
}

async fn read<T>(reader:&mut SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>> )->Result<T,String> where T:DeserializeOwned{
    let text_message: Message=reader.next().await.ok_or("invalid")?.map_err(|e|e.to_string())?;
    let result=match text_message{
        Message::Text(bytes)=>{
            let str=Utf8Bytes::to_string(&bytes);
            serde_json::from_str::<T>(&str).map_err(|e|e.to_string())?
        },
        _=>panic!()
    };
    Ok(result)

}