use std::{collections::HashMap, sync::Arc};

use domain::{
    db::{
        db_region_set::{DbRegionSet, RegionSetId},
        db_track::TrackId,
    },
    region_set::{
        copy_region_set_params::CopyRegionSetParams as DbCopyRegionSetParams,
        create_region_set_params::CreateRegionSetParams as DbCreateRegionSetParams,
        edit_region_set_params::EditRegionSetParams as DbEditRegionSetParams,
        region_set_subtree::RegionSetSubtree,
    },
    regions::region_subtree::RegionSubtree,
};

use super::{
    data_provider::region_sets_data_provider::RegionSetsDataProvider,
    region_sets_provider::{
        CopyRegionSetParams, CreateRegionSetParams, EditRegionSetParams, RegionSetsProvider,
    },
};

pub struct RegionSetsProviderService {
    data: Arc<dyn RegionSetsDataProvider>,
}

impl RegionSetsProviderService {
    pub fn new(data: Arc<dyn RegionSetsDataProvider>) -> Self {
        Self { data }
    }

    async fn load_region_set_subtree(
        &self,
        set_id: &RegionSetId,
    ) -> Result<RegionSetSubtree, String> {
        let set = self.data.get_region_set(set_id).await?;
        let regions = self.data.get_regions_for_region_set(&set.region_set_id).await?;

        Ok(RegionSetSubtree {
            track_id: set.track_id,
            track_length: set.track_length_seconds,
            region_set_id: set.region_set_id,
            name: set.name,
            regions: regions
                .into_iter()
                .map(|r| RegionSubtree {
                    region_id: r.region_id,
                    region_set_id: r.region_set_id,
                    name: r.name,
                    start_time: r.start_time_seconds,
                    end_time: r.end_time_seconds,
                    graph: None,
                })
                .collect(),
        })
    }
}

#[async_trait::async_trait]
impl RegionSetsProvider for RegionSetsProviderService {
    async fn create_region_set(
        &self,
        params: CreateRegionSetParams,
    ) -> Result<DbRegionSet, String> {
        let track = self.data.get_track(&params.track_id).await?;
        self.data
            .create_region_set(DbCreateRegionSetParams {
                track_id: params.track_id,
                track_length: track.length_seconds,
                name: params.name,
            })
            .await
    }

    async fn get_region_set(&self, set_id: &RegionSetId) -> Result<DbRegionSet, String> {
        self.data.get_region_set(set_id).await
    }

    async fn get_region_sets(&self) -> Result<HashMap<TrackId, Vec<DbRegionSet>>, String> {
        let sets = self.data.get_region_sets().await?;
        let mut map: HashMap<TrackId, Vec<DbRegionSet>> = HashMap::new();
        for set in sets {
            map.entry(set.track_id.clone()).or_default().push(set);
        }
        Ok(map)
    }

    async fn get_region_sets_for_track(
        &self,
        track_id: &TrackId,
    ) -> Result<Vec<DbRegionSet>, String> {
        self.data.get_region_sets_for_track(track_id).await
    }

    async fn edit_region_set(
        &self,
        params: EditRegionSetParams,
    ) -> Result<DbRegionSet, String> {
        self.data
            .edit_region_set(DbEditRegionSetParams {
                name: params.name,
                region_set_id: params.region_set_id,
                track_id: params.track_id,
            })
            .await
    }

    async fn delete_region_set(&self, set_id: &RegionSetId) -> Result<(), String> {
        self.data.delete_region_set(set_id).await
    }

    async fn copy_region_set(
        &self,
        params: CopyRegionSetParams,
    ) -> Result<RegionSetSubtree, String> {
        let new_set = self
            .data
            .copy_region_set(DbCopyRegionSetParams {
                region_set_id: params.region_set_id,
                region_set_name: params.copy_name,
            })
            .await?;
        self.load_region_set_subtree(&new_set.region_set_id).await
    }
}
