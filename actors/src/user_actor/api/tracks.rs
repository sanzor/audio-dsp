use domain::{
    actors::messages::crud::{
        copy_track::{CopyTrack, CopyTrackResult},
        delete_track::{DeleteTrack, DeleteTrackResult},
        get_track::{GetRawTrack, GetRawTrackResult},
        get_track_info::{GetTrackMeta, GetTrackMetaResult},
        get_tracks::{GetTrackMetas, GetTracksResult},
        insert_track::{InsertTrack, InsertTrackResult},
        update_track_info::{UpdateTrackInfo, UpdateTrackInfoResult},
    },
    track_meta::TrackMeta,
};

use crate::user_actor::user_actor::UserActor;
use kameo::prelude::Context;
use kameo::prelude::Message;

impl Message<GetRawTrack> for UserActor {
    type Reply = Result<GetRawTrackResult, String>;

    async fn handle(&mut self, msg: GetRawTrack, ctx: &mut Context<Self, Self::Reply>) -> Self::Reply {
        let track = self.tracks_provider.get_track_copy(&msg.track_id).await;
        match track {
            Err(e) => Err("Could not find track".to_string()),
            Ok(e) => Ok(GetRawTrackResult { track: e }),
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
        let raw_track_copy = self.tracks_provider.get_track_copy(&msg.track_id).await?;

        let insert_result = self.tracks_provider.upsert_track(raw_track_copy).await?;
        Ok(CopyTrackResult {
            copied_track_id: insert_result.track_id,
            track_copy_name: insert_result.track_info.name,
        })
    }
}

impl Message<GetTrackMeta> for UserActor {
    type Reply = Result<GetTrackMetaResult, String>;

    async fn handle(
        &mut self,
        msg: GetTrackMeta,
        ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        let get_info_result = self.tracks_provider.get_track_meta(&msg.track_id).await?;
        Ok(GetTrackMetaResult {
            track_meta: get_info_result,
        })
    }
}

impl Message<GetTrackMetas> for UserActor {
    type Reply = Result<GetTracksResult, String>;

    async fn handle(
        &mut self,
        msg: GetTrackMetas,
        ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        let tracks_info_result = self.tracks_provider.get_all_track_infos().await?;
        let result: Vec<TrackMeta> = tracks_info_result.track_infos.into_values().collect();
        Ok(GetTracksResult { tracks: result })
    }
}

impl Message<InsertTrack> for UserActor {
    type Reply = Result<InsertTrackResult, String>;

    async fn handle(
        &mut self,
        msg: InsertTrack,
        ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        let track_meta = self.tracks_provider.upsert_track(msg.track).await?;
        Ok(InsertTrackResult {
            track_id: track_meta.track_id.clone(),
            user_id: self.id.to_string(),
        })
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
        let update_result = self
            .tracks_provider
            .update_track_info(&msg.track_id,msg.track_info)
            .await?;
        Ok(UpdateTrackInfoResult {
            track_meta: update_result,
        })
    }
}
