use std::sync::Arc;

use domain::{
    db::db_region::DbRegion,
    region_set::region_set_subtree::RegionSetSubtree,
    regions::region_subtree::RegionSubtree,
    regions::{
        add_region_params::AddRegionParams as DbAddRegionParams,
        copy_region_params::CopyRegionParams as DbCopyRegionParams,
        delete_region_params::DeleteRegionParams as DbDeleteRegionParams,
        edit_region_params::EditRegionParams as DbEditRegionParams,
    },
};

use super::{
    data_provider::regions_data_provider::RegionsDataProvider,
    regions_provider::{
        AddRegionParams, CopyRegionParams, DeleteRegionParams, EditRegionParams, EndTimePolicy,
        RegionsProvider,
    },
};

pub struct RegionsProviderService {
    data: Arc<dyn RegionsDataProvider>,
}

impl RegionsProviderService {
    pub fn new(data: Arc<dyn RegionsDataProvider>) -> Self {
        Self { data }
    }

    async fn load_region_set_subtree(
        &self,
        set_id: &domain::db::db_region_set::RegionSetId,
    ) -> Result<RegionSetSubtree, String> {
        let set = self.data.get_region_set(set_id).await?;
        let regions = self
            .data
            .get_regions_for_region_set(&set.region_set_id)
            .await?;

        Ok(RegionSetSubtree {
            track_id: set.track_id,
            track_length: set.track_length_seconds,
            region_set_id: set.region_set_id,
            name: set.name,
            regions: regions
                .into_iter()
                .map(|region| RegionSubtree {
                    region_id: region.region_id,
                    region_set_id: region.region_set_id,
                    name: region.name,
                    start_time: region.start_time_seconds,
                    end_time: region.end_time_seconds,
                    graph: None,
                })
                .collect(),
        })
    }

    fn resolve_end_time(
        regions: &[DbRegion],
        track_length: f32,
        new_region_start_time: f32,
        end_time_policy: EndTimePolicy,
    ) -> Result<f32, String> {
        match end_time_policy {
            EndTimePolicy::Explicit(val) => {
                if val <= new_region_start_time {
                    Err("End time must be greater than start time".to_string())
                } else {
                    Ok(val)
                }
            }
            EndTimePolicy::NextRegionOrEnd => {
                let next_start = regions
                    .iter()
                    .filter(|r| r.start_time_seconds > new_region_start_time)
                    .map(|r| r.start_time_seconds)
                    .min_by(|a, b| a.partial_cmp(b).unwrap());
                Ok(next_start.unwrap_or(track_length))
            }
            EndTimePolicy::FixedLength(len) => Ok(new_region_start_time + len),
        }
    }
}

#[async_trait::async_trait]
impl RegionsProvider for RegionsProviderService {
    async fn add_region(&self, params: AddRegionParams) -> Result<RegionSetSubtree, String> {
        let set = self.data.get_region_set(&params.region_set_id).await?;
        let regions = self
            .data
            .get_regions_for_region_set(&params.region_set_id)
            .await?;
        let end_time = Self::resolve_end_time(
            &regions,
            set.track_length_seconds,
            params.start_time,
            params.end_time_policy,
        )?;
        self.data
            .add_region(DbAddRegionParams {
                name: params.name,
                end_time,
                start_time: params.start_time,
                region_set_id: params.region_set_id.clone(),
            })
            .await?;

        self.load_region_set_subtree(&params.region_set_id).await
    }

    async fn edit_region(&self, params: EditRegionParams) -> Result<DbRegion, String> {
        self.data
            .edit_region(DbEditRegionParams {
                name: params.name,
                region_id: params.region_id,
                end_time: params.end_time,
                start_time: params.start_time,
            })
            .await
    }

    async fn delete_region(&self, params: DeleteRegionParams) -> Result<RegionSetSubtree, String> {
        let region = self.data.get_region(&params.region_id).await?;
        let region_set_id = region.region_set_id;

        self.data
            .delete_region(DbDeleteRegionParams {
                region_id: params.region_id,
            })
            .await?;

        self.load_region_set_subtree(&region_set_id).await
    }

    async fn copy_region(&self, params: CopyRegionParams) -> Result<DbRegion, String> {
        self.data
            .copy_region(DbCopyRegionParams {
                source_region_set_id: params.source_region_set_id,
                source_region_id: params.source_region_id,
                source_track_id: params.source_track_id,
                destination_region_set_id: params.destination_region_set_id.clone(),
                destination_track_id: params.destination_track_id,
                region_copy_name: params.copy_name,
            })
            .await
    }
}
