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
    },
    regions::region::Region,
    regions::region_set::RegionSet,
    raw_track::TrackInfo,
    track_meta::TrackMeta,
};
use kameo::prelude::{Context, Message};

use crate::user_actor::actor::UserActor;

impl UserActor {
    pub async fn load_region_set_domain(&self, region_set_id: &str) -> Result<RegionSet, String> {
        let set: DbRegionSet = self.region_set_provider.get_region_set(&region_set_id.to_string()).await?;
        let regions: Vec<DbRegion> = self
            .region_set_provider
            .get_regions_for_region_set(&set.region_set_id)
            .await?;

        Ok(RegionSet {
            track_id: set.track_id.clone(),
            track_length: set.track_length_seconds,
            region_set_id: set.region_set_id.clone(),
            name: set.name.clone(),
            regions: regions
                .into_iter()
                .map(|r| Region {
                    region_set_id: r.region_set_id,
                    region_id: r.region_id,
                    name: r.name,
                    start_time: r.start_time_seconds,
                    end_time: r.end_time_seconds,
                    graph: None,
                })
                .collect(),
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
        let track_meta = TrackMeta {
            track_info: TrackInfo {
                name: track.name.clone(),
                extension: track.extension.clone(),
                length: track.length_seconds,
            },
            track_id: track.track_id.clone(),
        };
        let set: DbRegionSet = self
            .region_set_provider
            .create_region_set(CreateRegionSetParams {
                name: msg.name,
                track_id: msg.track_id,
                track_length: track_meta.track_info.length,
            })
            .await?;
        let region_set = self.load_region_set_domain(&set.region_set_id).await?;
        Ok(CreateRegionSetResult { region_set })
    }
}

impl Message<GetRegionSet> for UserActor {
    type Reply = Result<GetRegionSetResult, String>;

    async fn handle(
        &mut self,
        msg: GetRegionSet,
        _ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        let set = self.load_region_set_domain(&msg.region_set_id).await?;
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
        let mut map = std::collections::HashMap::new();
        for set in sets {
            let domain_set = self.load_region_set_domain(&set.region_set_id).await?;
            map.entry(domain_set.track_id.clone())
                .or_insert_with(Vec::new)
                .push(domain_set);
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
        let sets = self
            .region_set_provider
            .get_region_sets_for_track(&msg.track_id)
            .await?;
        let mut domain_sets = Vec::with_capacity(sets.len());
        for set in sets {
            domain_sets.push(self.load_region_set_domain(&set.region_set_id).await?);
        }
        Ok(GetRegionSetsForTrackResult {
            region_sets: domain_sets,
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
        let region_set = self.load_region_set_domain(&result.region_set_id).await?;
        Ok(EditRegionSetResult { region_set })
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
        let region_set = self.load_region_set_domain(&result.region_set_id).await?;
        Ok(CopyRegionSetResult { region_set })
    }
}
