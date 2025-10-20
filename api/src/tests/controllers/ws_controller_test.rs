use core::panic;
use std::{sync::Arc, time::Duration};

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
    tungstenite::{
        client::IntoClientRequest, handshake::client::Request, http, Message, Utf8Bytes,
    },
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
    player_controller_test::{
        controllers::utils::{create_user_and_actor, init_env, make_test_auth_cookie},
        utils::{get_user_state, insert_track, make_raw_track_from_samples},
    },
    user_and_actor_resolver::local_user_and_actor_resolver::LocalUserAndActorResolver,
};

#[rstest]
#[actix_web::test]
async fn can_start_player_ws() -> Result<(), String> {
    let track = make_raw_track_from_samples(vec![1_f32, 2_f32], Channels::Mono);
    init_env();
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET not set");
    let user_name = Ulid::new();
    let user_actor_deps = Arc::new(UserActorDeps {
        player_factory: Arc::new(PlayerFactory {}),
        tracks_provider: Arc::new(LocalTrackStoreProvider::new()),
        region_sets_provider: Arc::new(InMemoryRegionSetProvider::new()),
    });

    let user_provider: Arc<dyn UserProvider> = Arc::new(InMemoryUserProvider::new());
    let user_registry = Arc::new(UserActorRegistry::new());

    let (created_user, _) = create_user_and_actor(
        user_name.to_string(),
        user_registry.clone(),
        user_provider.clone(),
        user_actor_deps.clone(),
    )
    .await?;

    let app_data = AppData {
        user_actor_deps: Arc::new(UserActorDeps {
            player_factory: Arc::new(PlayerFactory {}),
            tracks_provider: Arc::new(LocalTrackStoreProvider::new()),
            region_sets_provider: Arc::new(InMemoryRegionSetProvider::new()),
        }),
        user_resolver: Arc::new(LocalUserAndActorResolver::new(user_provider, user_registry)),
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

    let _server_handle = actix_rt::spawn(bound.run());

    let mut app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_data))
            .service(web::scope("/tracks").configure(tracks_crud_controller::init)),
    )
    .await;
    let cookie = make_test_auth_cookie(secret, created_user.id.clone());
    let insert_result = insert_track(cookie.clone(), &mut app, AddTrackParams { track }).await?;

    let ws_url = format!(
        "ws://{}:{}/ws/run?user_id={}&track_id={}",
        addr.ip(),
        addr.port(),
        created_user.id,
        insert_result.track_id
    );

    // Create a raw tungstenite request from the URL
    let mut request: Request = ws_url.clone().into_client_request().unwrap();

    // Insert Cookie header (this is the only one we override)
    request.headers_mut().insert(
        http::header::COOKIE,
        format!("{}={}", cookie.name(), cookie.value())
            .parse()
            .unwrap(),
    );

    // Now connect using connect_async
    let (ws_stream, _) = connect_async(request).await.expect("Failed to connect");
    let (mut write, mut ws_reader) = ws_stream.split();

    let play_request = serde_json::to_string(&WsMessage::Play {
        track_id: insert_result.track_id,
    })
    .unwrap();

    let _v = write.send(Message::Text(play_request.into())).await;
    let msg = read::<WebsocketSendMessage>(&mut ws_reader).await?;
    let _frame = match msg {
        WebsocketSendMessage::AudioFrame { audio_frame } => audio_frame,
    };
    Ok(())
}

#[rstest]
#[actix_web::test]
async fn can_stop_player_ws() -> Result<(), String> {
    let track = make_raw_track_from_samples(vec![1_f32; 512], Channels::Mono);
    init_env();
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET not set");
    let user_name = Ulid::new();
    let user_actor_deps = Arc::new(UserActorDeps {
        player_factory: Arc::new(PlayerFactory {}),
        tracks_provider: Arc::new(LocalTrackStoreProvider::new()),
        region_sets_provider: Arc::new(InMemoryRegionSetProvider::new()),
    });

    let user_provider: Arc<dyn UserProvider> = Arc::new(InMemoryUserProvider::new());
    let user_registry = Arc::new(UserActorRegistry::new());

    let (created_user, _) = create_user_and_actor(
        user_name.to_string(),
        user_registry.clone(),
        user_provider.clone(),
        user_actor_deps.clone(),
    )
    .await?;

    let app_data = AppData {
        user_actor_deps: Arc::new(UserActorDeps {
            player_factory: Arc::new(PlayerFactory {}),
            tracks_provider: Arc::new(LocalTrackStoreProvider::new()),
            region_sets_provider: Arc::new(InMemoryRegionSetProvider::new()),
        }),
        user_resolver: Arc::new(LocalUserAndActorResolver::new(user_provider, user_registry)),
    };
    let cookie = make_test_auth_cookie(secret, created_user.id.clone());
    let url = "127.0.0.1:0";
    let server_app_data = app_data.clone();
    let sv = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(server_app_data.clone()))
            .service(web::scope("/ws").configure(ws_controller::init))
    });
    let bound = sv.bind(url).unwrap();
    let addr = bound.addrs()[0];

    let _server_handle = actix_rt::spawn(bound.run());

    let mut app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_data))
            .service(web::scope("/tracks").configure(tracks_crud_controller::init))
            .service(web::scope("/user").configure(user_controller::init)),
    )
    .await;

    let insert_result = insert_track(cookie.clone(), &mut app, AddTrackParams { track }).await?;

    let ws_url = format!(
        "ws://{}:{}/ws/run?user_id={}&track_id={}",
        addr.ip(),
        addr.port(),
        created_user.id,
        insert_result.track_id
    );

    // Create a raw tungstenite request from the URL
    let mut request: Request = ws_url.clone().into_client_request().unwrap();

    // Insert Cookie header (this is the only one we override)
    request.headers_mut().insert(
        http::header::COOKIE,
        format!("{}={}", cookie.name(), cookie.value())
            .parse()
            .unwrap(),
    );

    // Now connect using connect_async
    let (ws_stream, _) = connect_async(request).await.expect("Failed to connect");
    let (mut write, mut ws_reader) = ws_stream.split();

    let play_request = serde_json::to_string(&WsMessage::Play {
        track_id: insert_result.track_id.clone(),
    })
    .unwrap();

    let _v = write.send(Message::Text(play_request.into())).await;

    let msg = read::<WebsocketSendMessage>(&mut ws_reader).await?;
    let _frame = match msg {
        WebsocketSendMessage::AudioFrame { audio_frame } => audio_frame,
    };
    let pause_request = serde_json::to_string(&WsMessage::Pause {
        track_id: insert_result.track_id.clone(),
    })
    .unwrap();

    let _ = write.send(Message::Text(pause_request.into())).await;
    tokio::time::sleep(Duration::from_secs(1)).await;
    let user_state = get_user_state(cookie, &mut app, created_user.id).await?;
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
