use std::{collections::HashMap, sync::Arc};

use actors::audio_player_actor::{
    attach_sink::AttachSink,
    create_audio_player_actor_params::CreateAudioPlayerActorParams,
    player_track_payload::PlayerTrackPayload,
    registry::AudioPlayerRegistry,
};
use audiolib::utils::decode_canonical_audio;
use domain::{
    actors::messages::player::{
        get_player_state::{GetPlayerState, GetPlayerStateResult},
        pause::Pause,
        play::Play,
        remove_sink::RemoveSink,
        seek::Seek,
        stop::Stop,
    },
    db::TrackId,
};
use player::sink::AudioSink;

use crate::tracks::tracks_provider::TracksProvider;

use super::player_provider::{AttachSinkParams, AttachSinkResult, PlayerProvider};

pub struct PlayerService {
    registry: Arc<AudioPlayerRegistry>,
    tracks_provider: Arc<dyn TracksProvider>,
}

impl PlayerService {
    pub fn new(registry: Arc<AudioPlayerRegistry>, tracks_provider: Arc<dyn TracksProvider>) -> Self {
        Self { registry, tracks_provider }
    }

    fn player_key(user_id: domain::domain_user::UserId, track_id: TrackId) -> String {
        format!("{user_id}:{track_id}")
    }
}

#[async_trait::async_trait]
impl PlayerProvider for PlayerService {
    async fn play(&self, user_id: domain::domain_user::UserId, track_id: TrackId) -> Result<(), String> {
        let player = self
            .registry
            .get(&Self::player_key(user_id, track_id))
            .await
            .ok_or("Player not found")?;
        player.tell(Play {}).await.map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn pause(&self, user_id: domain::domain_user::UserId, track_id: TrackId) -> Result<(), String> {
        let player = self
            .registry
            .get(&Self::player_key(user_id, track_id))
            .await
            .ok_or("Player not found")?;
        player.tell(Pause {}).await.map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn seek(&self, user_id: domain::domain_user::UserId, track_id: TrackId, position: u32) -> Result<(), String> {
        let player = self
            .registry
            .get(&Self::player_key(user_id, track_id))
            .await
            .ok_or("Player not found")?;
        player.tell(Seek { position }).await.map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn stop(&self, user_id: domain::domain_user::UserId, track_id: TrackId) -> Result<(), String> {
        let key = Self::player_key(user_id, track_id);
        let player = self.registry.get(&key).await.ok_or("Player not found")?;
        player.tell(Stop {}).await.map_err(|e| e.to_string())?;
        self.registry.remove(&key).await;
        Ok(())
    }

    async fn attach_sink(&self, params: AttachSinkParams) -> Result<AttachSinkResult, String> {
        let key = Self::player_key(params.user_id, params.track_id);

        if let Some(player) = self.registry.get(&key).await {
            let result = player
                .ask(AttachSink { sink: params.sink })
                .await
                .map_err(|e| e.to_string())?;
            return Ok(AttachSinkResult {
                sink_id: result.sink_id,
                track_id: params.track_id,
            });
        }

        let track = self
            .tracks_provider
            .get_track(&params.track_id, params.workspace_id)
            .await?;
        let decoded = decode_canonical_audio(&track.payload.canonical_audio)?;

        let sink_id = ulid::Ulid::new().to_string();
        let mut sinks: HashMap<String, Box<dyn AudioSink + Sync + Send>> = HashMap::new();
        sinks.insert(sink_id.clone(), params.sink);

        self.registry
            .insert(
                key,
                CreateAudioPlayerActorParams {
                    track_payload: PlayerTrackPayload { audio: decoded, meta: track.meta },
                    cursor: 0,
                    sinks,
                },
            )
            .await;

        Ok(AttachSinkResult {
            sink_id,
            track_id: params.track_id,
        })
    }

    async fn remove_sink(&self, user_id: domain::domain_user::UserId, track_id: TrackId, sink_id: &str) -> Result<(), String> {
        let player = self
            .registry
            .get(&Self::player_key(user_id, track_id))
            .await
            .ok_or("Player not found")?;
        player
            .ask(RemoveSink { sink_id: sink_id.to_string() })
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn get_state(&self, user_id: domain::domain_user::UserId, track_id: TrackId) -> Result<GetPlayerStateResult, String> {
        let player = self
            .registry
            .get(&Self::player_key(user_id, track_id))
            .await
            .ok_or("Player not found")?;
        player.ask(GetPlayerState {}).await.map_err(|e| e.to_string())
    }
}
