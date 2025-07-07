use std::collections::HashMap;

use domain::{
    actors::messages::{
        crud::{
            copy_track::{CopyTrack, CopyTrackResult},
            delete_track::{DeleteTrack, DeleteTrackResult},
            get_track::{GetTrack, GetTrackResult},
            get_track_info::{GetTrackInfo, GetTrackInfoResult},
            get_tracks::{GetTracks, GetTracksResult},
            insert_track::{InsertTrack, InsertTrackResult},
            update_track_info::{UpdateTrackInfo, UpdateTrackInfoResult},
        },
        player::get_player_state::{GetPlayerState, PlayerStateResult},
        user::get_user_state::{GetUserState, GetUserStateResult},
    },
    track::TrackInfo,
};

use crate::user_actor::user_actor::UserActor;
use kameo::prelude::Context;
use kameo::prelude::Message;

impl Message<GetTrack> for UserActor {
    type Reply = Result<GetTrackResult, String>;

    async fn handle(&mut self, msg: GetTrack, ctx: &mut Context<Self, Self::Reply>) -> Self::Reply {
        let track = self.tracks_provider.get_track_copy(&msg.track_id).await;
        match track {
            Err(e) => Err("Could not find track".to_string()),
            Ok(e) => Ok(GetTrackResult { track: e }),
        }
    }
}

impl Message<CopyTrack> for UserActor {
    type Reply = Result<CopyTrackResult, String>;

    async fn handle(
        &mut self,
        msg: CopyTrack,
        ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        let source = self.tracks_provider.get_track_ref(&msg.track_id).await?;
        let mut clone = source.inner.clone();
        clone.info.name = msg.track_copy_name;
        let insert_result = self.tracks_provider.upsert_track(clone).await;
        match insert_result {
            Err(e) => Err("Could not find track".to_string()),
            Ok(()) => Ok(CopyTrackResult {
                copied_track_id: clone.info.name,
                track_copy_name: clone.info.name,
            }),
        }
    }
}

impl Message<GetTrackInfo> for UserActor {
    type Reply = Result<GetTrackInfoResult, String>;

    async fn handle(
        &mut self,
        msg: GetTrackInfo,
        ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        let get_info_result = self.tracks_provider.get_track_info(&msg.track_id).await;
        get_info_result.map(|e|GetTrackInfoResult{track_info:e})
    }
}

impl Message<GetTracks> for UserActor {
    type Reply = Result<GetTracksResult, String>;

    async fn handle(
        &mut self,
        msg: GetTracks,
        ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        let tracks = self.tracks_provider.get_all_track_infos().await;
        tracks.map(|result|GetTracksResult { tracks: result.track_infos.values().collect() })
    }
}

impl Message<InsertTrack> for UserActor {
    type Reply = Result<InsertTrackResult, String>;

    async fn handle(
        &mut self,
        msg: InsertTrack,
        ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        let track = self.tracks_provider.upsert_track(msg.track).await;
        track.map(|()| InsertTrackResult {})
    }
}

impl Message<DeleteTrack> for UserActor {
    type Reply = Result<DeleteTrackResult, String>;

    async fn handle(
        &mut self,
        msg: DeleteTrack,
        ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        let del_result = self.tracks_provider.delete_track(&msg.track_id).await;
        del_result.map(|()| DeleteTrackResult {})
    }
}

impl Message<UpdateTrackInfo> for UserActor {
    type Reply = Result<UpdateTrackInfoResult, String>;

    async fn handle(
        &mut self,
        msg: UpdateTrackInfo,
        ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        let update_result = self.tracks_provider.update_track_info(msg.track_info).await;
        update_result.map(|()| UpdateTrackInfoResult {})
    }
}

impl Message<GetUserState> for UserActor {
    type Reply = Result<GetUserStateResult, String>;

    async fn handle(
        &mut self,
        msg: GetUserState,
        ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        let mut player_list: HashMap<String, PlayerStateResult> = HashMap::new();
        let mut track_list: HashMap<String, TrackInfo> = HashMap::new();
        let players_result = self.players_provider.get_all().await?;
        let tracks_result = self.tracks_provider.get_all_track_infos().await?;
        for (key, track_info) in tracks_result.track_infos {
            track_list.insert(key, track_info);
        }
        for element in players_result.items {
            let player_state: PlayerStateResult = element.player_ref.ask(GetPlayerState {}).await?;
            player_list.insert(element.player_id, player_state.state);
        }
        Ok(GetUserStateResult {
            tracks: track_list,
            players: player_list,
        })
    }
}
