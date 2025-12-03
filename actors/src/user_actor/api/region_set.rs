use domain::{
    actors::messages::region_set::{
        copy_region_set::{CopyRegionSet, CopyRegionSetResult},
        create_region_set::{CreateRegionSet, CreateRegionSetResult},
        delete_region_set::{DeleteRegionSet, DeleteRegionSetResult},
        edit_region_set::{EditRegionSet, EditRegionSetResult},
        get_region_set::{GetRegionSet, GetRegionSetResult},
        get_region_sets_for_track::{GetRegionSetsForTrack, GetRegionSetsForTrackResult},
        get_regions_sets::{GetRegionSets, GetRegionSetsResult},
    },
    region_set::{
        copy_region_set_params::CopyRegionSetParams,
        create_region_set_params::CreateRegionSetParams,
        edit_region_set_params::EditRegionSetParams,
    },
    regions::region_set::RegionSet,
    track_meta::TrackMeta,
};
use kameo::prelude::{Context, Message};

use crate::user_actor::actor::UserActor;

impl Message<CreateRegionSet> for UserActor {
    type Reply = Result<CreateRegionSetResult, String>;

    async fn handle(
        &mut self,
        msg: CreateRegionSet,
        _ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        let track_meta: TrackMeta = self.tracks_provider.get_track_meta(&msg.track_id).await?;
        let set: RegionSet = self
            .region_set_provider
            .create_region_set(CreateRegionSetParams {
                name: msg.name,
                track_id: msg.track_id,
                track_length: track_meta.track_info.length,
            })
            .await?;
        Ok(CreateRegionSetResult { region_set: set })
    }
}

impl Message<GetRegionSet> for UserActor {
    type Reply = Result<GetRegionSetResult, String>;

    async fn handle(
        &mut self,
        msg: GetRegionSet,
        _ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        let set: RegionSet = self
            .region_set_provider
            .get_region_set(&msg.region_set_id)
            .await?;
        Ok(GetRegionSetResult { region_set: set })
    }
}

impl Message<GetRegionSets> for UserActor {
    type Reply = Result<GetRegionSetsResult, String>;

    async fn handle(
        &mut self,
        _msg: GetRegionSets,
        _ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        let sets = self.region_set_provider.get_region_sets().await?;
        Ok(GetRegionSetsResult {
            track_region_sets_map: sets,
        })
    }
}

impl Message<GetRegionSetsForTrack> for UserActor {
    type Reply = Result<GetRegionSetsForTrackResult, String>;

    async fn handle(
        &mut self,
        msg: GetRegionSetsForTrack,
        _ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        let sets = self
            .region_set_provider
            .get_region_sets_for_track(&msg.track_id)
            .await?;
        Ok(GetRegionSetsForTrackResult {
            region_sets: sets,
            track_id: msg.track_id,
        })
    }
}

impl Message<EditRegionSet> for UserActor {
    type Reply = Result<EditRegionSetResult, String>;

    async fn handle(
        &mut self,
        msg: EditRegionSet,
        _ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        let result = self
            .region_set_provider
            .edit_region_set(EditRegionSetParams {
                name: msg.name,
                region_set_id: msg.region_set_id,
                track_id: msg.track_id,
            })
            .await?;
        Ok(EditRegionSetResult { region_set: result })
    }
}

impl Message<DeleteRegionSet> for UserActor {
    type Reply = Result<DeleteRegionSetResult, String>;

    async fn handle(
        &mut self,
        msg: DeleteRegionSet,
        _ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        self.region_set_provider
            .delete_region_set(&msg.region_set_id)
            .await?;
        Ok(DeleteRegionSetResult {})
    }
}

impl Message<CopyRegionSet> for UserActor {
    type Reply = Result<CopyRegionSetResult, String>;

    async fn handle(
        &mut self,
        msg: CopyRegionSet,
        _ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        let result = self
            .region_set_provider
            .copy_region_set(CopyRegionSetParams {
                region_set_id: msg.region_set_id,
                region_set_name: msg.region_set_copy_name,
            })
            .await?;
        Ok(CopyRegionSetResult { region_set: result })
    }
}
