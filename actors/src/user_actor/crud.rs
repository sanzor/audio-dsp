
use domain::actors::messages::
    {crud::{delete_track::{DeleteTrackParams, DeleteTrackResult}, get_track::{GetTrackParams, GetTrackResult},
    get_track_info::{GetTrackInfo, GetTrackInfoResult},
    insert_track::InsertTrack, update_track_info::UpdateTrackInfoParams}, player::get_player_state::GetPlayerStateResult, user_to_player::get_player_state::{GetPlayerState, GetPlayerStateResult}};

use dsp_core::state::{TracksProvider, TracksState};
use kameo::prelude::{Context, Message};
use crate::user_actor::user_actor::UserActor;


#[async_trait::async_trait]
impl TrackOperations for LocalTracksProvider{
    async fn handle_insert_track(&mut self,track:Track)->Result<InsertTrackResult,String> {
        let rez=self.track_state.upsert_track(track).await;
        Ok(InsertTrackResult{})
    }

    async fn handle_delete(&mut self,track_id:String)->Result<DeleteTrackResult,String> {
        let rez=self.track_state.delete_track(&track_id).await;
        Ok(DeleteTrackResult{})
    }

    async fn handle_update(&mut self,params:UpdateTrackInfoParams)->Result<UpdateTrackInfoResult,String> {
        let rez=self.track_state.update_track_info(params.track_info).await;
    }

    async fn get_track(&self,track_id:String)->Result<Track,String> {
       let track=self.track_state.get_track_copy(&track_id).await;
       track
    }

    async fn get_track_info(&self,track_id:String)->Result<GetTrackInfoResult,String> {
        let info=self.track_state.get_track_info(&track_id).await?;
        Ok(GetTrackInfoResult{track_info:info})
    }
    
    async fn get_state(&self) -> Result<GetUserStateResult,String> {
        let mut player_list: HashMap<String, PlayerStateQueryResult> = HashMap::new();
        let mut track_list: HashMap<String, TrackInfo> = HashMap::new();

        for (key, player_ref) in self.players.iter() {
            let player_state = self.handle_get_player_state(Some(key.into())).await?;
            player_list.insert(key.into(), player_state);
        }
        for (key, track) in self.track_state.tracks.iter() {
            track_list.insert(key.into(), track.info.clone());
        }

        Ok(GetUserStateResult {
            players: player_list,
            tracks: track_list,
        })
    }

    async fn get_player_state(&self,get_params:GetPlayerState)->Result<GetPlayerStateResult,String>{

    }
}

impl Message<InsertTrack> for UserActor {
    type Reply = Result<InsertTrackResult, String>;

    async fn handle(
        &mut self,
        msg: InsertTrack,
        ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        let c=self.tracks_provider.handle_insert_track(msg.track).await;
        c
    }
}

impl Message<UpdateTrackInfoParams> for UserActor {
    type Reply = Result<UpdateTrackInfoResult, String>;

    async fn handle(
        &mut self,
        msg: UpdateTrackInfoParams,
        ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        let c=self.tracks_provider.handle_update(msg).await;
        c
    }
}

impl Message<DeleteTrackParams> for UserActor{
    type Reply = Result<DeleteTrackResult, String>;

    async fn handle(
        &mut self,
        msg: DeleteTrackParams,
        ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        let c=self.tracks_provider.handle_delete(msg.track_id).await;
        c
    }
}

impl Message<GetTrackInfo> for UserActor{
    type Reply = Result<GetTrackInfoResult, String>;

    async fn handle(
        &mut self,
        msg: GetTrackInfo,
        ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        let c=self.tracks_provider.get_track_info(msg.track_id).await;
        c
    }
}

impl Message<GetTrackParams> for UserActor{
    type Reply = Result<GetTrackResult, String>;

    async fn handle(
        &mut self,
        msg: GetTrackParams,
        ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        let c=self.tracks_provider.get_track(msg.track_id).await.map(|t|GetTrackResult { track: t });
        c
    }
}

impl Message<GetPlayerState> for UserActor{
    type Reply=Result<GetPlayerStateResult,String>;

    async fn handle(
        &mut self,
        msg: GetPlayerState,
        ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        let s=self.tracks_provider.GE().await;
        s
    }
}