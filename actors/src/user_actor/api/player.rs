use audiolib::utils::decode_canonical_audio;
use domain::actors::messages::player::get_player_state::GetPlayerState;
use domain::actors::messages::player::pause::Pause;
use domain::actors::messages::player::play::Play;
use domain::actors::messages::player::remove_sink::RemoveSink;
use domain::actors::messages::player::{seek::Seek, stop::Stop};
use domain::actors::messages::user_to_player::user_get_player_state::UserGetPlayerStateResult;
use domain::actors::messages::user_to_player::user_pause::UserPauseResult;
use domain::actors::messages::user_to_player::user_play::UserPlayResult;
use domain::actors::messages::user_to_player::user_seek::UserSeekResult;
use domain::actors::messages::user_to_player::user_stop::UserStopResult;
use domain::actors::messages::user_to_player::{
    user_get_player_state::UserGetPlayerState, user_pause::UserPause, user_play::UserPlay,
    user_seek::UserSeek, user_stop::UserStop,
};
use domain::stored_track::StoredTrack;
use kameo::actor::ActorRef;
use kameo::prelude::{Context, Message};
use std::collections::HashMap;
use std::sync::Arc;

use crate::audio_player_actor::attach_sink::AttachSink;
use crate::audio_player_actor::audio_player_actor::AudioPlayerActor;
use crate::audio_player_actor::create_audio_player_actor_params::CreateAudioPlayerActorParams;

use crate::audio_player_actor::player_track_payload::PlayerTrackPayload;
use crate::user_actor::user_actor::UserActor;
use crate::user_actor::user_attach_sink::{UserAttachSink, UserAttachSinkResult};
use crate::user_actor::user_remove_sink::{UserRemoveSink, UserRemoveSinkResult};

impl Message<UserPlay> for UserActor {
    type Reply = Result<UserPlayResult, String>;

    async fn handle(&mut self, msg: UserPlay, ctx: &mut Context<Self, Self::Reply>) -> Self::Reply {
        let player_result = self.players_provider.get(&msg.track_id.clone());

        let res = match player_result {
            Some(player) => player.tell(Play {}).await,
            None => return Err("Could not find player".to_string()),
        };
        Ok(UserPlayResult {})
    }
}

impl Message<UserPause> for UserActor {
    type Reply = Result<UserPauseResult, String>;

    async fn handle(
        &mut self,
        msg: UserPause,
        ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        if let Some(player) = self.players_provider.get(&msg.track_id) {
            player.tell(Pause {}).await.unwrap();
            Ok(UserPauseResult {})
        } else {
            Err("Could not find player".into())
        }
    }
}

impl Message<UserStop> for UserActor {
    type Reply = Result<UserStopResult, String>;

    async fn handle(&mut self, msg: UserStop, ctx: &mut Context<Self, Self::Reply>) -> Self::Reply {
        if let Some(player) = self.players_provider.get(&msg.track_id.clone()) {
            player.tell(Stop {}).await.unwrap();
            let removed_player = self.players_provider.remove(&msg.track_id);
            if let Some(pl) = removed_player {
                drop(pl);
            }
            Ok(UserStopResult {})
        } else {
            Err("Could not find player".into())
        }
    }
}

impl Message<UserSeek> for UserActor {
    type Reply = Result<UserSeekResult, String>;

    async fn handle(&mut self, msg: UserSeek, ctx: &mut Context<Self, Self::Reply>) -> Self::Reply {
        if let Some(player) = self.players_provider.get(&msg.track_id) {
            player
                .tell(Seek {
                    position: msg.position,
                })
                .await
                .unwrap();
            Ok(UserSeekResult {})
        } else {
            Err("Could not find player".into())
        }
    }
}
impl Message<UserAttachSink> for UserActor {
    type Reply = Result<UserAttachSinkResult, String>;

    async fn handle(
        &mut self,
        msg: UserAttachSink,
        ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        let result = match self.players_provider.get(&msg.track_id.clone()) {
            Some(player) => {
                self.handle_attach_sink_to_existing_player(msg, &player)
                    .await
            }
            None => self.handle_attach_sink_to_new_player(msg).await,
        };

        result
    }
}

impl Message<UserRemoveSink> for UserActor {
    type Reply = Result<UserRemoveSinkResult, String>;

    async fn handle(
        &mut self,
        msg: UserRemoveSink,
        ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        if let Some(player_result) = self.players_provider.get(&msg.track_id.clone()) {
            match player_result
                .ask(RemoveSink {
                    sink_id: msg.sink_id,
                })
                .await
                .map_err(|e| e.to_string())
            {
                Ok(e) => Ok(UserRemoveSinkResult {}),
                Err(e) => Err(e.to_string()),
            }
        } else {
            Err("Could not get player".into())
        }
    }
}
impl Message<UserGetPlayerState> for UserActor {
    type Reply = Result<UserGetPlayerStateResult, String>;

    async fn handle(
        &mut self,
        msg: UserGetPlayerState,
        ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        let player = self.players_provider.get(&msg.track_id);
        match player {
            None => Err("Could not find player".into()),
            Some(p) => {
                let x: domain::actors::messages::player::get_player_state::GetPlayerStateResult =
                    p.ask(GetPlayerState {}).await.map_err(|e| e.to_string())?;
                Ok(UserGetPlayerStateResult {
                    cursor: x.cursor,
                    written: x.written,
                    state: x.state,
                    sinks: x.sinks,
                })
            }
        }
    }
}

impl UserActor {
    async fn get_stored_track(&mut self, track_id: &str) -> Result<Arc<StoredTrack>, String> {
        let payload = match self.cached_tracks.get(track_id) {
            Some(payload) => Arc::clone(&payload),
            None => {
                let track_copy: StoredTrack =
                    self.tracks_provider.get_stored_track(track_id).await?;

                let payload_ref = Arc::new(track_copy);
                self.cached_tracks
                    .insert(track_id.to_string(), Arc::clone(&payload_ref));
                payload_ref
                // let payload_ref = Arc::new(track_copy);
                // self.loaded_payloads
                //     .insert(track_id.to_string(), Arc::clone(&payload_ref));
                // payload_ref
            }
        };
        Ok(payload)
    }
    async fn handle_attach_sink_to_new_player(
        &mut self,
        msg: UserAttachSink,
    ) -> Result<UserAttachSinkResult, String> {
        let meta = self.tracks_provider.get_track_meta(&msg.track_id).await?;
        let sink_id = ulid::Ulid::new().to_string();
        let mut sinks = HashMap::new();
        sinks.insert(sink_id.clone(), msg.sink);
        let payload = self.get_stored_track(&msg.track_id).await?;
        let decoded_track = decode_canonical_audio(&payload.canonical_audio)?;
        let create_audio_actor_params = CreateAudioPlayerActorParams {
            track_payload: PlayerTrackPayload {
                audio: decoded_track,
                meta: meta,
            },
            cursor: 0,
            sinks: sinks,
        };

        let create_actor_result = self
            .player_factory
            .create_audio_actor(create_audio_actor_params)?;

        match self.players_provider.insert(
            msg.track_id.to_string(),
            create_actor_result.audio_actor_ref,
        ) {
            None => Ok(UserAttachSinkResult {
                sink_id: sink_id,
                track_id: msg.track_id,
            }),
            Some(v) => Err("key alrdy present".into()),
        }
    }
    async fn handle_attach_sink_to_existing_player(
        &self,
        msg: UserAttachSink,
        player_ref: &ActorRef<AudioPlayerActor>,
    ) -> Result<UserAttachSinkResult, String> {
        match player_ref.ask(AttachSink { sink: msg.sink }).await {
            Err(e) => Err(e.to_string()),

            Ok(r) => Ok(UserAttachSinkResult {
                track_id: msg.track_id,
                sink_id: r.sink_id,
            }),
        }
    }
}
