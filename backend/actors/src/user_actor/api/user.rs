use std::collections::HashMap;

use domain::{
    actors::messages::{
        player::get_player_state::{GetPlayerState, GetPlayerStateResult},
        user::{
            get_user_state::{GetUserState, GetUserStateResult},
            remove_user::{RemoveUser, RemoveUserResult},
            update_user::{UpdateUser, UpdateUserResult},
        },
    },
    db::db_track::DbTrack,
    raw_track::TrackInfo,
    track_meta::TrackMeta,
};
use kameo::prelude::{Context, Message};

use crate::user_actor::actor::UserActor;

fn db_track_to_track_meta(track: &DbTrack) -> TrackMeta {
    TrackMeta {
        track_info: TrackInfo {
            name: track.name.clone(),
            extension: track.extension.clone(),
            length: track.length_seconds,
        },
        track_id: track.track_id.clone(),
    }
}

impl Message<GetUserState> for UserActor {
    type Reply = Result<GetUserStateResult, String>;

    async fn handle(
        &mut self,
        _msg: GetUserState,
        _ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        let mut player_list: HashMap<String, GetPlayerStateResult> = HashMap::new();
        let mut track_list: HashMap<String, TrackMeta> = HashMap::new();

        let tracks = self.tracks_provider.get_all_tracks().await?;
        for track in tracks {
            let meta = db_track_to_track_meta(&track);
            track_list.insert(meta.track_id.clone(), meta);
        }
        for (k, element) in &self.players_provider {
            let player_state = element
                .ask(GetPlayerState {})
                .await
                .map_err(|e| e.to_string())?;
            player_list.insert(k.to_string(), player_state);
        }
        Ok(GetUserStateResult {
            tracks: track_list,
            players: player_list,
        })
    }
}

impl Message<RemoveUser> for UserActor {
    type Reply = Result<RemoveUserResult, String>;

    async fn handle(
        &mut self,
        _msg: RemoveUser,
        _ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        Ok(RemoveUserResult {})
    }
}

impl Message<UpdateUser> for UserActor {
    type Reply = Result<UpdateUserResult, String>;

    async fn handle(
        &mut self,
        msg: UpdateUser,
        _ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        self.user_data.email = msg.email;
        self.user_data.name = msg.name;
        Ok(UpdateUserResult {
            id: self.user_data.id.clone(),
            new_email: self.user_data.email.clone(),
            new_name: self.user_data.name.clone(),
        })
    }
}
