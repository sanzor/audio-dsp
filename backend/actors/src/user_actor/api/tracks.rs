use domain::{
    actors::messages::tracks::{
        copy_track::{CopyTrack, CopyTrackResult},
        delete_track::{DeleteTrack, DeleteTrackResult},
        get_stored_track::{GetStoredTrack, GetStoredTrackResult},
        get_track_info::{GetTrackMeta, GetTrackMetaResult},
        get_tracks::{GetTrackMetas, GetTracksResult},
        insert_track::{InsertTrack, InsertTrackResult},
        update_track_info::{UpdateTrackInfo, UpdateTrackInfoResult},
    },
    db::db_track::DbTrack,
    raw_track::TrackInfo,
    stored_track::StoredTrack,
    track_meta::TrackMeta,
    update_track_info_params::UpdateTrackInfoParams,
};

use crate::user_actor::actor::UserActor;
use kameo::prelude::Context;
use kameo::prelude::Message;

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

fn db_track_to_stored_track(track: DbTrack) -> StoredTrack {
    StoredTrack {
        track_id: track.track_id,
        track_info: TrackInfo {
            name: track.name,
            extension: track.extension,
            length: track.length_seconds,
        },
        canonical_audio: track.canonical_audio,
    }
}

impl Message<GetStoredTrack> for UserActor {
    type Reply = Result<GetStoredTrackResult, String>;

    async fn handle(
        &mut self,
        msg: GetStoredTrack,
        _ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        let track = self.tracks_provider.get_track(&msg.track_id).await?;
        Ok(GetStoredTrackResult {
            track: db_track_to_stored_track(track),
        })
    }
}

impl Message<CopyTrack> for UserActor {
    type Reply = Result<CopyTrackResult, String>;

    async fn handle(
        &mut self,
        msg: CopyTrack,
        _ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        let result = self
            .tracks_provider
            .copy_track(&msg.track_id, &msg.track_copy_name)
            .await?;
        let result_meta = db_track_to_track_meta(&result);
        Ok(CopyTrackResult {
            copied_track_id: result_meta.track_id,
            track_copy_name: result_meta.track_info.name,
        })
    }
}

impl Message<GetTrackMeta> for UserActor {
    type Reply = Result<GetTrackMetaResult, String>;

    async fn handle(
        &mut self,
        msg: GetTrackMeta,
        _ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        let get_info_result = self.tracks_provider.get_track(&msg.track_id).await?;
        Ok(GetTrackMetaResult {
            track_meta: db_track_to_track_meta(&get_info_result),
        })
    }
}

impl Message<GetTrackMetas> for UserActor {
    type Reply = Result<GetTracksResult, String>;

    async fn handle(
        &mut self,
        _msg: GetTrackMetas,
        _ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        let tracks = self.tracks_provider.get_all_tracks().await?;
        let result: Vec<TrackMeta> = tracks.iter().map(db_track_to_track_meta).collect();
        Ok(GetTracksResult { tracks: result })
    }
}

impl Message<InsertTrack> for UserActor {
    type Reply = Result<InsertTrackResult, String>;

    async fn handle(
        &mut self,
        msg: InsertTrack,
        _ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        let track = self.tracks_provider.upsert_track(msg.track).await?;
        let track_meta = db_track_to_track_meta(&track);
        Ok(InsertTrackResult {
            track_id: track_meta.track_id.clone(),
            user_id: self.user_data.id.to_string(),
        })
    }
}

impl Message<DeleteTrack> for UserActor {
    type Reply = Result<DeleteTrackResult, String>;

    async fn handle(
        &mut self,
        msg: DeleteTrack,
        _ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        self.tracks_provider.delete_track(&msg.track_id).await?;
        Ok(DeleteTrackResult {})
    }
}

impl Message<UpdateTrackInfo> for UserActor {
    type Reply = Result<UpdateTrackInfoResult, String>;

    async fn handle(
        &mut self,
        msg: domain::actors::messages::tracks::update_track_info::UpdateTrackInfo,
        _ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        let update_track = self
            .tracks_provider
            .update_track_info(
                &msg.track_id,
                UpdateTrackInfoParams {
                    track_name: msg.name,
                },
            )
            .await?;
        Ok(UpdateTrackInfoResult {
            track_meta: db_track_to_track_meta(&update_track),
        })
    }
}
