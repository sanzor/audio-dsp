use dsp_core::{command_processor::CommandProcessor, state::TracksState};
use kameo::{
    actor::ActorRef,
    prelude::{Context, Message},
    spawn, Actor,
};
use player::audio_sink::cpal_sink::CpalSink;
use std::{collections::HashMap, sync::Arc};

use crate::audio_player_actor::{
    audio_player_actor::AudioPlayerActor, audio_player_actor_params::AudioPlayerActorParams,
};
use domain::{
    actors::{
        player_command::PlayerCommand, player_state_query::PlayerStateQuery,
        player_state_query_result::PlayerStateQueryResult, user_player_command::UserPlayerCommand,
        user_player_command_result::UserPlayerCommandResult,
        user_player_state_query::UserPlayerStateQuery,
        user_player_state_query_result::UserPlayerStateQueryResult,
        user_state_query::UserStateQuery, user_state_query_result::UserStateQueryResult,
    },
    dsp_message::DspMessage,
    track::TrackInfo,
    tracks_message_result::TracksMessageResult,
};

type Players = HashMap<String, ActorRef<AudioPlayerActor>>;

#[derive(Actor)]
#[actor(name = "UserActor")]
pub struct UserActor {
    id:String,
    name:String,
    processor: Arc<CommandProcessor>,
    track_state: TracksState,
    players: Players,
}
pub struct UserActorParams{
    pub id:String,
    pub name:String,
    pub processor: Arc<CommandProcessor>,
    pub track_state: TracksState,
    pub players: Players,
}
impl UserActor {
    pub fn new(actor_params:UserActorParams) -> UserActor {
        UserActor {
            id:actor_params.id,
            name:actor_params.name,
            processor: actor_params.processor,
            track_state: actor_params.track_state,
            players: actor_params.players,
        }
    }
}

impl Message<DspMessage> for UserActor {
    type Reply = Result<TracksMessageResult, String>;

    // async move variant
    async fn handle(
        &mut self,
        msg: DspMessage,
        ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        let v = self
            .processor
            .process_crud_command(msg, &mut self.track_state)
            .await;
        v
    }
}

impl Message<UserPlayerCommand> for UserActor {
    type Reply = Result<UserPlayerCommandResult, String>;
    async fn handle(
        &mut self,
        msg: UserPlayerCommand,
        ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        let c = match msg {
            UserPlayerCommand::Play { track_id } => self.handle_play(track_id).await,
            UserPlayerCommand::Pause { track_id } => self.handle_pause(track_id).await,
            UserPlayerCommand::Stop { track_id } => self.handle_stop(track_id).await,
            UserPlayerCommand::Seek { track_id, position } => {
                self.handle_seek(track_id, position).await
            }
        };
        c
    }
}
impl Message<UserPlayerStateQuery> for UserActor {
    type Reply = Result<UserPlayerStateQueryResult, String>;

    async fn handle(
        &mut self,
        msg: UserPlayerStateQuery,
        ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        let c = self.handle_get_player_state(msg.track_id).await?;
        Ok(UserPlayerStateQueryResult {
            cursor: c.cursor,
            state: c.state,
            written: c.written,
        })
    }
}
impl Message<UserStateQuery> for UserActor {
    type Reply = Result<UserStateQueryResult, String>;

    async fn handle(
        &mut self,
        msg: UserStateQuery,
        ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        let mut player_list: HashMap<String, PlayerStateQueryResult> = HashMap::new();
        let mut track_list: HashMap<String, TrackInfo> = HashMap::new();

        for (key, player_ref) in self.players.iter() {
            let player_state = self.handle_get_player_state(Some(key.into())).await?;
            player_list.insert(key.into(), player_state);
        }
        for (key, track) in self.track_state.tracks.iter() {
            track_list.insert(key.into(), track.info.clone());
        }

        Ok(UserStateQueryResult {
            players: player_list,
            tracks: track_list,
        })
    }
}
impl UserActor {
    pub async fn handle_play(
        &mut self,
        track_id: Option<String>,
    ) -> Result<UserPlayerCommandResult, String> {
        let track_id = track_id.ok_or_else(|| "invalid id".to_string())?;
        let player = self.players.get(&track_id);
        match player.cloned() {
            None => self.handle_play_new_player(&track_id).await,
            Some(p) => self.handle_play_existing_player(&p).await,
        }
    }
    async fn handle_pause(
        &mut self,
        track_id: Option<String>,
    ) -> Result<UserPlayerCommandResult, String> {
        let track_id = track_id.ok_or_else(|| "invalid id".to_string())?;
        if let Some(player) = self.players.get(&track_id) {
            player.tell(PlayerCommand::Pause {}).await.unwrap();
            Ok(UserPlayerCommandResult {
                should_exit: false,
                output: "Paused player".into(),
            })
        } else {
            Err("Could not find player".into())
        }
    }
    async fn handle_stop(
        &mut self,
        track_id: Option<String>,
    ) -> Result<UserPlayerCommandResult, String> {
        let track_id = track_id.ok_or_else(|| "invalid id".to_string())?;
        if let Some(player) = self.players.get(&track_id) {
            player.tell(PlayerCommand::Stop {}).await.unwrap();
            let removed_player = self.players.remove(&track_id);
            if let Some(pl) = removed_player {
                drop(pl);
            }
            Ok(UserPlayerCommandResult {
                should_exit: false,
                output: "Paused stopped".into(),
            })
        } else {
            Err("Could not find player".into())
        }
    }

    async fn handle_seek(
        &mut self,
        track_id: Option<String>,
        position: u32,
    ) -> Result<UserPlayerCommandResult, String> {
        let track_id = track_id.ok_or_else(|| "invalid id".to_string())?;
        if let Some(player) = self.players.get(&track_id) {
            player
                .tell(PlayerCommand::Seek { position: position })
                .await
                .unwrap();
            Ok(UserPlayerCommandResult {
                should_exit: false,
                output: "Player moved at position".into(),
            })
        } else {
            Err("Could not find player".into())
        }
    }

    pub async fn handle_get_player_state(
        &self,
        track_id: Option<String>,
    ) -> Result<PlayerStateQueryResult, String> {
        let track_id = track_id.ok_or_else(|| "invalid id".to_string())?;
        dbg!("got here");
        let player = self.players.get(&track_id);
        match player.cloned() {
            None => Err("Player does not exist".to_string()),
            Some(p) => {
                let x = p
                    .ask(PlayerStateQuery {})
                    .await
                    .map_err(|e| e.to_string())?;
                Ok(x)
            }
        }
    }

    async fn handle_play_new_player(
        &mut self,
        track_id: &str,
    ) -> Result<UserPlayerCommandResult, String> {
        let track_ref = self.track_state.get_track_ref(track_id).await?;
        let sink = Box::new(CpalSink::new()?);
        let params = AudioPlayerActorParams {
            track: track_ref.inner.clone(),
            cursor: 0,
            sink: sink,
        };
        let player_actor = spawn(AudioPlayerActor::new(params));
        let play_result = player_actor.tell(PlayerCommand::Play {}).await.unwrap();
        if let Some(x) = self.players.insert(track_id.to_string(), player_actor) {
            Err("Could not insert ".into())
        } else {
            Ok(UserPlayerCommandResult {
                should_exit: false,
                output: "Inserted succesfully ".into(),
            })
        }
    }
    async fn handle_play_existing_player(
        &mut self,
        player_ref: &ActorRef<AudioPlayerActor>,
    ) -> Result<UserPlayerCommandResult, String> {
        if player_ref.tell(PlayerCommand::Play {}).await.is_err() {
            Err("Could not start player".into())
        } else {
            Ok(UserPlayerCommandResult {
                should_exit: false,
                output: "Started player".into(),
            })
        }
    }
}
