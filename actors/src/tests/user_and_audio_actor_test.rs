use std::{collections::{HashMap, VecDeque}, sync::Arc};

use audiolib::{audio_buffer::AudioBuffer, Channels};
use domain::{
    actors::{
        messages::{
            crud::insert_track::InsertTrack,
            user::get_user_state::{GetUserState, GetUserStateResult},
            user_to_player::{
                user_get_player_state::UserGetPlayerState, user_pause::UserPause,
                user_play::UserPlay, user_stop::UserStop,
            },
        },
        player_state::AudioPlayerState,
        user_player_state_query_result::UserPlayerStateQueryResult,
    },
    raw_track::{RawTrack, TrackInfo},
};

use dsp_core::tracks_provider::LocalTrackStoreProvider;
use kameo::{actor::ActorRef, Actor};
use player::{audio_sink::{queue_sink::QueueSink, AudioSink}, AudioFrame};
use tokio::sync::Mutex;
use ulid::Ulid;

use crate::user_actor::{
    create_user_actor_params::CreateUserActorParams, create_user_data::CreateUserData,
    local_players_provider::LocalPlayerProvider, player_factory::PlayerFactory,
    user_actor::UserActor, user_attach_sink::{UserAttachSink, UserAttachSinkResult}, user_remove_sink::{UserRemoveSink, UserRemoveSinkResult},
};
struct TestSink{
    pub queue:Arc<Mutex<VecDeque<AudioFrame>>>
}
impl AudioSink for TestSink{
    fn write_frame<'a>(
        &'a mut self,
        frame: AudioFrame,
    ) -> std::pin::Pin<Box<dyn std::prelude::rust_2024::Future<Output = Result<(), String>> + Send + 'a>> {
        todo!()
    }
}

fn create_user_actor(id: Ulid) -> ActorRef<UserActor> {
    let tracks_provider = Box::new(LocalTrackStoreProvider::new());
    let players_provider = Box::new(LocalPlayerProvider::new());
    let actor_params = CreateUserActorParams {
        user_data: CreateUserData {
            email: id.to_string(),
            name: id.to_string(),
            id: id.to_string(),
        },
        players_provider: players_provider,
        tracks_provider,
        player_factory: Arc::new(PlayerFactory {}),
    };
    let actor = UserActor::spawn(UserActor::new(actor_params));
    let g = kameo::registry::ActorRegistry::new();

    actor
}


#[tokio::test]
async fn can_attach_sink_to_player() -> Result<(), String> {
    let track_id = "some_track";
    let track = sample_track(track_id);
    let user_name = "my_user".to_string();
    let id = Ulid::new();
    let user_actor = create_user_actor(id);
    let insert_track_result = user_actor
        .ask(InsertTrack { track: track })
        .await
        .map_err(|e| e.to_string())?;
    let sink=QueueSink{queue:Arc::new(Mutex::new(VecDeque::new()))};
    let attach_sink_result: UserAttachSinkResult=user_actor.ask(UserAttachSink{sink:Box::new(sink),track_id:insert_track_result.track_id.clone()})
               .await.map_err(|e|e.to_string())?;
    
    let user_actor_state_result = get_user_state(&user_actor).await?;
    let sinks=user_actor_state_result.players.get(track_id).unwrap().clone().sinks;
    let attached_sink_exists=sinks.iter().find(|s|**s==attach_sink_result.sink_id).is_some();
    assert_eq!(sinks.len(),1);
    assert!(attached_sink_exists);
    assert!(matches!(user_actor_state_result.players.len(), 1));
   
    Ok(())
}

#[tokio::test]
async fn can_create_player_and_play() -> Result<(), String> {
    let track_name = "some_track";
    let track = sample_track(track_name);
    let user_name = "my_user".to_string();
    let id = Ulid::new();
    let user_actor = create_user_actor(id);
    let insert_result = user_actor
        .ask(InsertTrack { track: track })
        .await
        .map_err(|e| e.to_string())?;
    let sink=QueueSink{queue:Arc::new(Mutex::new(VecDeque::new()))};
    let attach_result: UserAttachSinkResult=user_actor.ask(UserAttachSink{sink:Box::new(sink),track_id:insert_result.track_id.clone()})
               .await.map_err(|e|e.to_string())?;
    let play = user_actor
        .tell(UserPlay {
            track_id: insert_result.track_id.clone(),
        })
        .await
        .map_err(|e| e.to_string())?;
    let user_actor_state_result = get_player_state(&user_actor, &insert_result.track_id).await?;

    assert!(matches!(
        user_actor_state_result.state,
        AudioPlayerState::Playing
    ));
    Ok(())
}


#[tokio::test]
async fn can_play_on_existing_player() -> Result<(), String> {
    let track_name = "some_track";
    let track = sample_track(track_name);
    let user_name = "my_user".to_string();
    let id = Ulid::new();
    let user_actor = create_user_actor(id);
    let insert_result = user_actor
        .ask(InsertTrack { track: track })
        .await
        .map_err(|e| e.to_string())?;
    let sink=QueueSink{queue:Arc::new(Mutex::new(VecDeque::new()))};
    let attach_result: UserAttachSinkResult=user_actor.ask(UserAttachSink{sink:Box::new(sink),track_id:insert_result.track_id.clone()})
               .await.map_err(|e|e.to_string())?;
    let play = user_actor
        .tell(UserPlay {
            track_id: insert_result.track_id.clone(),
        })
        .await;
    let user_actor_state_result = get_user_state(&user_actor).await?;
    assert!(matches!(user_actor_state_result.players.len(), 1));
    assert!(matches!(
        user_actor_state_result
            .players
            .get(&insert_result.track_id.clone())
            .unwrap()
            .state,
        AudioPlayerState::Playing
    ));
    let pause = user_actor
        .tell(UserPause {
            track_id: insert_result.track_id.clone(),
        })
        .await;
    let play_again = user_actor
        .tell(UserPlay {
            track_id: insert_result.track_id,
        })
        .await;
    let user_actor_state_result = get_user_state(&user_actor).await?;
    assert_eq!(user_actor_state_result.players.len(), 1);
    Ok(())
}

#[tokio::test]
async fn can_create_player_and_stop() -> Result<(), String> {
    let track_name = "some_track";
    let track = sample_track(track_name);
    let user_name = "my_user".to_string();
    let id = Ulid::new();
    let user_actor = create_user_actor(id);
    let insert_result = user_actor
        .ask(InsertTrack { track: track })
        .await
        .map_err(|e| e.to_string())?;
    let sink=QueueSink{queue:Arc::new(Mutex::new(VecDeque::new()))};
    let attach_result: UserAttachSinkResult=user_actor.ask(UserAttachSink{sink:Box::new(sink),track_id:insert_result.track_id.clone()})
               .await.map_err(|e|e.to_string())?;
    let player_id = insert_result.track_id.clone();
    let play = user_actor
        .tell(UserPlay {
            track_id: insert_result.track_id.into(),
        })
        .await;
    assert!(play.is_ok());
    let user_actor_state_result = get_player_state(&user_actor, &player_id.clone()).await?;

    assert!(matches!(
        user_actor_state_result.state,
        AudioPlayerState::Playing
    ));
    let play = user_actor
        .tell(UserStop {
            track_id: player_id.clone(),
        })
        .await;

    let state = get_player_state(&user_actor, &player_id.clone()).await;
    assert!(state.unwrap_err().contains("Could not find player"));
    let deleted = get_user_state(&user_actor).await?;
    assert!(deleted.players.len() == 0);
    Ok(())
}

fn sample_track(track_name: &str) -> RawTrack {
    let samples = vec![1.1_f32; 500];
    let sample_rate = 1_f32;
    let track = RawTrack {
        info: TrackInfo {
            name: track_name.to_string(),
        },
        data: AudioBuffer {
            channels: Channels::Mono,
            samples: samples,
            sample_rate: sample_rate,
        },
    };
    track
}
fn to_string(track: &RawTrack) -> String {
    serde_json::to_string(track).unwrap()
}

async fn get_player_state(
    user_actor: &ActorRef<UserActor>,
    track_id: &str,
) -> Result<UserPlayerStateQueryResult, String> {
    let rez = user_actor
        .ask(UserGetPlayerState {
            track_id: track_id.into(),
        })
        .await
        .map_err(|e| e.to_string())?;

    Ok(UserPlayerStateQueryResult {
        cursor: rez.cursor,
        written: rez.written,
        state: rez.state,
        sinks:rez.sinks
    })
}
async fn get_user_state(user_actor: &ActorRef<UserActor>) -> Result<GetUserStateResult, String> {
    let rez = user_actor
        .ask(GetUserState {})
        .await
        .map_err(|e| e.to_string())?;

    Ok(rez)
}

async fn attach_sink(user_actor:&ActorRef<UserActor>,track_id:String,sink:Box<dyn AudioSink+Send+Sync>)->Result<UserAttachSinkResult,String>{
    let params=UserAttachSink{
        sink:sink,
        track_id:track_id
    };
    let rez=user_actor.ask(params).await.map_err(|e|e.to_string())?;
    Ok(rez)
}

async fn remove_sink(user_actor:&ActorRef<UserActor>,sink_id:String,track_id:String)->Result<UserRemoveSinkResult,String>{
    let params=UserRemoveSink{
        sink_id:sink_id,
        track_id:track_id
    };
    let rez=user_actor.ask(params).await.map_err(|e|e.to_string())?;
    Ok(rez)
}