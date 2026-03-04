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
    db::db_region::DbRegion,
    db::db_region_set::DbRegionSet,
    db::db_track::DbTrack,
    region_set::{
        copy_region_set_params::CopyRegionSetParams,
        create_region_set_params::CreateRegionSetParams,
        edit_region_set_params::EditRegionSetParams,
        region_set_subtree::RegionSetSubtree,
    },
    regions::region_subtree::RegionSubtree,
};
use kameo::prelude::{Context, Message};

use crate::user_actor::actor::UserActor;

impl UserActor {
    pub async fn load_region_set_subtree(
        &self,
        region_set_id: &str,
    ) -> Result<RegionSetSubtree, String> {
        let set: DbRegionSet = self
            .region_set_provider
            .get_region_set(&region_set_id.to_string())
            .await?;
        let regions: Vec<DbRegion> = self
            .region_set_provider
            .get_regions_for_region_set(&set.region_set_id)
            .await?;

        Ok(RegionSetSubtree {
            track_id: set.track_id.clone(),
            track_length: set.track_length_seconds,
            region_set_id: set.region_set_id.clone(),
            name: set.name.clone(),
            regions: regions.into_iter().map(|_r| RegionSubtree {}).collect(),
        })
    }
}

impl Message<CreateRegionSet> for UserActor {
    type Reply = Result<CreateRegionSetResult, String>;

    async fn handle(
        &mut self,
        msg: CreateRegionSet,
        _ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        let track: DbTrack = self.tracks_provider.get_track(&msg.track_id).await?;
        let set: DbRegionSet = self
            .region_set_provider
            .create_region_set(CreateRegionSetParams {
                name: msg.name,
                track_id: msg.track_id,
                track_length: track.length_seconds,
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
        let set: DbRegionSet = self
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
        let sets: Vec<DbRegionSet> = self.region_set_provider.get_region_sets().await?;
        let mut map = std::collections::HashMap::new();
        for set in sets {
            map.entry(set.track_id.clone())
                .or_insert_with(Vec::new)
                .push(set);
        }
        Ok(GetRegionSetsResult {
            track_region_sets_map: map,
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
        let sets: Vec<DbRegionSet> = self
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
        let result: DbRegionSet = self
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
        let result: DbRegionSet = self
            .region_set_provider
            .copy_region_set(CopyRegionSetParams {
                region_set_id: msg.region_set_id,
                region_set_name: msg.region_set_copy_name,
            })
            .await?;
        let region_set_subtree = self.load_region_set_subtree(&result.region_set_id).await?;
        Ok(CopyRegionSetResult { region_set_subtree })
    }
}
