use std::collections::HashMap;

use data_provider::region_set_provider::RegionSetsProvider;
use domain::region_set::{
    copy_region_set_params::CopyRegionSetParams, create_region_set_params::CreateRegionSetParams,
    edit_region_set_params::EditRegionSetParams,
};
use dtos::db::{region_set_db_dto::RegionSetDbDto, region_set_subtree::RegionSetSubtree};

// 2. The Provider holds an Arc over the Pool
pub struct SqlxRegionSetDbProvider {}

impl SqlxRegionSetDbProvider {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait::async_trait]
impl RegionSetsProvider for SqlxRegionSetDbProvider {
    async fn create_region_set(
        &self,
        params: CreateRegionSetParams,
    ) -> Result<RegionSetDbDto, String> {
        let _ = (self, params);
        todo!()
    }

    async fn get_region_set(&self, set_id: &str) -> Result<RegionSetDbDto, String> {}

    async fn get_region_sets_for_track(
        &self,
        track_id: &str,
    ) -> Result<Vec<RegionSetDbDto>, String> {
        let _ = (self, track_id);
        todo!()
    }

    async fn get_region_sets(&self) -> Result<HashMap<String, Vec<RegionSetDbDto>>, String> {
        let _ = self;
        todo!()
    }

    async fn edit_region_set(&self, params: EditRegionSetParams) -> Result<RegionSetDbDto, String> {
        let _ = (self, params);
        todo!()
    }

    async fn delete_region_set(&self, set_id: &str) -> Result<(), String> {
        let _ = (self, set_id);
        todo!()
    }

    async fn copy_region_set(
        &self,
        params: CopyRegionSetParams,
    ) -> Result<RegionSetSubtree, String> {
        let _ = (self, params);
        todo!()
    }

    async fn fetch_subtree(&self, id: String) -> Result<RegionSetSubtree, String> {
        let _ = (self, id);
        todo!()
    }
}
