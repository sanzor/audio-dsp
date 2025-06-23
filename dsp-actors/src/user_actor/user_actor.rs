use dsp_core::{command_processor::CommandProcessor, state::TracksState};
use kameo::{
    actor::ActorRef,
    prelude::{Context, Message},
    spawn, Actor,
};
use player::audio_sink::cpal_sink::CpalSink;
use std::collections::HashMap;

use crate::audio_player_actor::{
    audio_player_actor::AudioPlayerActor, audio_player_actor_params::AudioPlayerActorParams,
};
use dsp_domain::{
    actors::{
        player_state_query::PlayerStateQuery, player_state_query_result::PlayerStateQueryResult,
        user_player_command::UserPlayerCommand,
        user_player_command_result::UserPlayerCommandResult,
        user_player_state_query::UserPlayerStateQuery,
        user_player_state_query_result::UserPlayerStateQueryResult,
        user_state_query_result::UserStateQueryResult,
    },
    dsp_message::DspMessage,
    track::TrackInfo,
    tracks_message_result::TracksMessageResult,
};

type Players = HashMap<String, ActorRef<AudioPlayerActor>>;

#[derive(Actor)]
#[actor(name = "UserActor")]
pub struct UserActor {
    processor: CommandProcessor,
    track_state: TracksState,
    players: Players,
}

impl UserActor {
    pub fn new(processor: CommandProcessor, tracks: TracksState, players: Players) -> UserActor {
        UserActor {
            processor: processor,
            track_state: tracks,
            players: players,
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
impl UserActor {
    pub async fn handle_play(
        &mut self,
        track_id: Option<String>,
    ) -> Result<UserPlayerCommandResult, String> {
        let track_id = track_id.ok_or_else(|| "invalid id".to_string())?;
        let player = self.players.get(&track_id);
        match player.cloned() {
            None => self.handle_new_player(&track_id).await,
            Some(p) => self.handle_play_existing_player(&p).await,
        }
    }
    async fn handle_pause(
        &mut self,
        track_id: Option<String>,
    ) -> Result<UserPlayerCommandResult, String> {
        let track_id = track_id.ok_or_else(|| "invalid id".to_string())?;
        let track_ref = self.track_state.get_track_ref(&track_id).await?;
        todo!()
    }
    async fn handle_stop(
        &mut self,
        track_id: Option<String>,
    ) -> Result<UserPlayerCommandResult, String> {
        let track_id = track_id.ok_or_else(|| "invalid id".to_string())?;
        let track_ref = self.track_state.get_track_ref(&track_id).await?;
        todo!()
    }

    async fn handle_seek(
        &mut self,
        track_id: Option<String>,
        position: u32,
    ) -> Result<UserPlayerCommandResult, String> {
        let track_id = track_id.ok_or_else(|| "invalid id".to_string())?;
        let track_ref = self.track_state.get_track_ref(&track_id).await?;
        todo!()
    }

    async fn handle_get_state(&mut self) -> Result<UserStateQueryResult, String> {
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

    pub async fn handle_get_player_state(
        &self,
        track_id: Option<String>,
    ) -> Result<PlayerStateQueryResult, String> {
        let track_id = track_id.ok_or_else(|| "invalid id".to_string())?;
        let player = self.players.get(&track_id);
        match player.cloned() {
            None => Err("no such player exists".to_string()),
            Some(p) => {
                let x = p
                    .ask(PlayerStateQuery {})
                    .await
                    .map_err(|e| e.to_string())?;
                Ok(x)
            }
        }
    }

    async fn handle_new_player(
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
        if let Some(x) = self.players.insert(track_id.to_string(), player_actor) {
            Ok(UserPlayerCommandResult {
                should_exit: false,
                output: "Inserted".into(),
            })
        } else {
            Err("Could not insert ".into())
        }
    }
    async fn handle_play_existing_player(
        &mut self,
        player_ref: &ActorRef<AudioPlayerActor>,
    ) -> Result<UserPlayerCommandResult, String> {
        todo!()
    }
}
