use core::panic;
use std::{collections::HashMap, sync::Arc, time::Duration};

use actix_web::{
    test,
    web::{self},
    App, HttpServer,
};

use actors::user_actor::{
    player_factory::PlayerFactory, user_actor_deps::UserActorDeps,
    user_actor_registry::UserActorRegistry,
};
use audiolib::Channels;
use data_provider::{
    in_memory_region_set_provider::InMemoryRegionSetProvider,
    in_memory_user_provider::InMemoryUserProvider, tracks_provider::LocalTrackStoreProvider,
    user_provider::UserProvider,
};
use domain::actors::player_state::AudioPlayerState;
use futures_util::{stream::SplitStream, SinkExt, StreamExt};
use rstest::rstest;
use serde::de::DeserializeOwned;
use tokio::net::TcpStream;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{Message, Utf8Bytes},
    MaybeTlsStream, WebSocketStream,
};
use ulid::Ulid;

use crate::{
    app_data::AppData,
    controllers::{
        tracks_crud_controller::{self, AddTrackParams},
        user_controller,
        ws_controller::{self, WebsocketSendMessage, WsMessage},
    },
    player_controller_test::utils::{
        create_user_actor, get_user_state, insert_track, make_raw_track_from_samples,
    },
    user_and_actor_resolver::local_user_and_actor_resolver::LocalUserAndActorResolver,
};

#[rstest]
#[actix_web::test]
async fn can_start_player_ws() -> Result<(), String> {
    let track = make_raw_track_from_samples(vec![1_f32, 2_f32], Channels::Mono);
    let id = Ulid::new();
    let user = create_user_actor(id);
    let user_provider: Arc<dyn UserProvider> = Arc::new(InMemoryUserProvider::new());
    let mut user_map = HashMap::new();

    user_map.insert(id.to_string(), user);
    let app_data = AppData {
        user_actor_deps: Arc::new(UserActorDeps {
            player_factory: Arc::new(PlayerFactory {}),
            tracks_provider: Arc::new(LocalTrackStoreProvider::new()),
            region_sets_provider: Arc::new(InMemoryRegionSetProvider::new()),
        }),
        user_resolver: Arc::new(LocalUserAndActorResolver::new(
            user_provider,
            Arc::new(UserActorRegistry::new()),
        )),
    };
    let url = "127.0.0.1:0";
    let server_app_data = app_data.clone();
    let sv = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(server_app_data.clone()))
            .service(web::scope("/ws").configure(ws_controller::init))
            .service(web::scope("/tracks").configure(tracks_crud_controller::init))
    });
    let bound = sv.bind(url).unwrap();
    let addr = bound.addrs()[0];

    let server_handle = actix_rt::spawn(bound.run());

    let mut app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_data))
            .service(web::scope("/tracks").configure(tracks_crud_controller::init)),
    )
    .await;

    let insert_result = insert_track(&mut app, AddTrackParams { track: track }).await?;

    let url = format!(
        "ws://{}:{}/ws/run?user_id={}&track_id={}",
        addr.ip(),
        addr.port(),
        id,
        insert_result.track_id
    );

    let (ws_stream, _) = connect_async(&url).await.expect("Failed to connect");

    let (mut write, mut ws_reader) = ws_stream.split();

    let play_request = serde_json::to_string(&WsMessage::Play {
        track_id: insert_result.track_id,
    })
    .unwrap();

    let v = write.send(Message::Text(play_request.into())).await;
    let msg = read::<WebsocketSendMessage>(&mut ws_reader).await?;
    let frame = match msg {
        WebsocketSendMessage::AudioFrame { audio_frame } => audio_frame,
        _ => panic!(),
    };
    Ok(())
}

#[rstest]
#[actix_web::test]
async fn can_stop_player_ws() -> Result<(), String> {
    let track = make_raw_track_from_samples(vec![1_f32; 512], Channels::Mono);
    let user_id = Ulid::new();
    let user = create_user_actor(user_id);
    let user_provider: Arc<dyn UserProvider> = Arc::new(InMemoryUserProvider::new());
    let mut user_map = HashMap::new();
    user_map.insert(user_id.to_string(), user);
    let app_data = AppData {
        user_actor_deps: Arc::new(UserActorDeps {
            player_factory: Arc::new(PlayerFactory {}),
            tracks_provider: Arc::new(LocalTrackStoreProvider::new()),
            region_sets_provider: Arc::new(InMemoryRegionSetProvider::new()),
        }),
        user_resolver: Arc::new(LocalUserAndActorResolver::new(
            user_provider,
            Arc::new(UserActorRegistry::new()),
        )),
    };
    let url = "127.0.0.1:0";
    let server_app_data = app_data.clone();
    let sv = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(server_app_data.clone()))
            .service(web::scope("/ws").configure(ws_controller::init))
    });
    let bound = sv.bind(url).unwrap();
    let addr = bound.addrs()[0];

    let server_handle = actix_rt::spawn(bound.run());

    let mut app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_data))
            .service(web::scope("/tracks").configure(tracks_crud_controller::init))
            .service(web::scope("/user").configure(user_controller::init)),
    )
    .await;

    let insert_result = insert_track(&mut app, AddTrackParams { track: track }).await?;
    let _ = get_user_state(&mut app, user_id.to_string()).await?;
    let ws_url = format!(
        "ws://{}:{}/ws/run?user_id={}&track_id={}",
        addr.ip(),
        addr.port(),
        user_id.to_string(),
        insert_result.track_id
    );

    let (ws_stream, _) = connect_async(&ws_url).await.expect("Failed to connect");

    let (mut write, mut ws_reader) = ws_stream.split();
    let user_state = get_user_state(&mut app, user_id.to_string()).await?;
    let play_request = serde_json::to_string(&WsMessage::Play {
        track_id: insert_result.track_id.clone(),
    })
    .unwrap();

    let v = write.send(Message::Text(play_request.into())).await;

    let msg = read::<WebsocketSendMessage>(&mut ws_reader).await?;
    let frame = match msg {
        WebsocketSendMessage::AudioFrame { audio_frame } => audio_frame,
        _ => panic!(),
    };
    let pause_request = serde_json::to_string(&WsMessage::Pause {
        track_id: insert_result.track_id.clone(),
    })
    .unwrap();

    let _ = write.send(Message::Text(pause_request.into())).await;
    tokio::time::sleep(Duration::from_secs(1)).await;
    let user_state = get_user_state(&mut app, user_id.to_string()).await?;
    let player = user_state.players.get(&insert_result.track_id).unwrap();
    assert!(matches!(player.state, AudioPlayerState::Paused));
    Ok(())
}
async fn read<T>(
    reader: &mut SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
) -> Result<T, String>
where
    T: DeserializeOwned,
{
    let text_message: Message = reader
        .next()
        .await
        .ok_or("invalid")?
        .map_err(|e| e.to_string())?;
    let result = match text_message {
        Message::Text(bytes) => {
            let str = Utf8Bytes::to_string(&bytes);
            serde_json::from_str::<T>(&str).map_err(|e| e.to_string())?
        }
        _ => panic!(),
    };
    Ok(result)
}
