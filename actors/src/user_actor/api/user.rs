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
    track_meta::TrackMeta,
};
use kameo::prelude::{Context, Message};

use crate::user_actor::user_actor::UserActor;

impl Message<GetUserState> for UserActor {
    type Reply = Result<GetUserStateResult, String>;

    async fn handle(
        &mut self,
        msg: GetUserState,
        ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        let mut player_list: HashMap<String, GetPlayerStateResult> = HashMap::new();
        let mut track_list: HashMap<String, TrackMeta> = HashMap::new();
       
        let tracks_result = self.tracks_provider.get_all_track_infos().await?;
        for (key, track_info) in tracks_result.track_infos {
            track_list.insert(key, track_info);
        }
        for (k,element) in &self.players_provider {
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
        msg: RemoveUser,
        ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        Ok(RemoveUserResult {})
    }
}

impl Message<UpdateUser> for UserActor {
    type Reply = Result<UpdateUserResult, String>;

    async fn handle(
        &mut self,
        msg: UpdateUser,
        ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        self.email = msg.email;
        self.name = msg.name;
        Ok(UpdateUserResult {
            id: self.id.clone(),
            new_email: self.email.clone(),
            new_name: self.name.clone(),
        })
    }
}
