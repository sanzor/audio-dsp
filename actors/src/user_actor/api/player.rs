use std::sync::Arc;

use audiolib::audio_buffer::AudioBuffer;
use domain::actors::messages::player::get_player_state::GetPlayerState;
use domain::actors::messages::player::pause::Pause;
use domain::actors::messages::player::play::Play;
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
use kameo::actor::ActorRef;
use kameo::prelude::{Context, Message};
use player::audio_sink::cpal_sink::CpalSink;

use crate::audio_player_actor::audio_player_actor::AudioPlayerActor;
use crate::audio_player_actor::create_audio_player_actor_params::CreateAudioPlayerActorParams;

use crate::user_actor::user_actor::UserActor;

impl Message<UserPlay> for UserActor {
    type Reply = Result<UserPlayResult, String>;

    async fn handle(&mut self, msg: UserPlay, ctx: &mut Context<Self, Self::Reply>) -> Self::Reply {
        let player_result = self.players_provider.get(msg.player_id.clone()).await;
        match player_result {
            Ok(player) => self.handle_play_existing_player(&player.player_ref).await,
            Err(err) => self.handle_play_new_player(&msg.player_id).await,
        }
    }
}

impl Message<UserPause> for UserActor {
    type Reply = Result<UserPauseResult, String>;

    async fn handle(
        &mut self,
        msg: UserPause,
        ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        if let Ok(player) = self.players_provider.get(msg.track_id).await {
            player.player_ref.tell(Pause {}).await.unwrap();
            Ok(UserPauseResult {})
        } else {
            Err("Could not find player".into())
        }
    }
}

impl Message<UserStop> for UserActor {
    type Reply = Result<UserStopResult, String>;

    async fn handle(&mut self, msg: UserStop, ctx: &mut Context<Self, Self::Reply>) -> Self::Reply {
        if let Ok(player) = self.players_provider.get(msg.track_id.clone()).await {
            player.player_ref.tell(Stop {}).await.unwrap();
            let removed_player = self.players_provider.remove(msg.track_id).await;
            if let Ok(pl) = removed_player {
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
        if let Ok(player) = self.players_provider.get(msg.track_id).await {
            player
                .player_ref
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

impl Message<UserGetPlayerState> for UserActor {
    type Reply = Result<UserGetPlayerStateResult, String>;

    async fn handle(
        &mut self,
        msg: UserGetPlayerState,
        ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        let player = self.players_provider.get(msg.track_id).await;
        match player {
            Err(e) => Err(e),
            Ok(p) => {
                let x = p
                    .player_ref
                    .ask(GetPlayerState {})
                    .await
                    .map_err(|e| e.to_string())?;
                Ok(UserGetPlayerStateResult {
                    cursor: x.cursor,
                    written: x.written,
                    state: x.state,
                })
            }
        }
    }
}

impl UserActor {
    async fn get_payload(&mut self, track_id: &str) -> Result<Arc<AudioBuffer>, String> {
        let payload = match self.loaded_payloads.get(track_id) {
            Some(payload) => Arc::clone(&payload),
            None => {
                let track_copy = self.tracks_provider.get_track_copy(track_id).await?;
                let payload_ref = Arc::new(track_copy.data);
                self.loaded_payloads
                    .insert(track_id.to_string(), Arc::clone(&payload_ref));
                payload_ref
            }
        };
        Ok(payload)
    }
    async fn handle_play_new_player(&mut self, track_id: &str) -> Result<UserPlayResult, String> {
        let meta = self.tracks_provider.get_track_meta(track_id).await?;

        let sink = Box::new(CpalSink::new()?);
        let payload = self.get_payload(track_id).await?;
        let create_audio_actor_params = CreateAudioPlayerActorParams {
            track_payload: payload,
            cursor: 0,
            sink: sink,
            meta: meta,
        };

        let create_actor_result = self
            .player_factory
            .create_audio_actor(create_audio_actor_params)?;

        let _ = create_actor_result
            .audio_actor_ref
            .tell(Play {})
            .await
            .unwrap();
        if let Ok(()) = self
            .players_provider
            .store(track_id.to_string(), create_actor_result.audio_actor_ref)
            .await
        {
            Ok(UserPlayResult {})
        } else {
            Err("Could not insert ".into())
        }
    }
    async fn handle_play_existing_player(
        &mut self,
        player_ref: &ActorRef<AudioPlayerActor>,
    ) -> Result<UserPlayResult, String> {
        if player_ref.tell(Play {}).await.is_err() {
            Err("Could not start player".into())
        } else {
            Ok(UserPlayResult {})
        }
    }
}
