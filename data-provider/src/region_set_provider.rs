use std::collections::HashMap;

use domain::region_set::{
    copy_region_set_params::CopyRegionSetParams, create_region_set_params::CreateRegionSetParams,
    edit_region_set_params::EditRegionSetParams,
};
use dtos::db::{region_set_db_dto::RegionSetDbDto, region_set_subtree::RegionSetSubtree};

#[async_trait::async_trait]
pub trait RegionSetsProvider: Send + Sync {
    // Aggregate-level CRUD
    async fn create_region_set(
        &self,
        params: CreateRegionSetParams,
    ) -> Result<RegionSetDbDto, String>;

    async fn get_region_set(&self, set_id: &str) -> Result<RegionSetDbDto, String>;

    async fn get_region_sets_for_track(
        &self,
        track_id: &str,
    ) -> Result<Vec<RegionSetDbDto>, String>;

    async fn get_region_sets(&self) -> Result<HashMap<String, Vec<RegionSetDbDto>>, String>;

    async fn edit_region_set(&self, params: EditRegionSetParams) -> Result<RegionSetDbDto, String>;

    async fn delete_region_set(&self, set_id: &str) -> Result<(), String>;

    async fn copy_region_set(
        &self,
        params: CopyRegionSetParams,
    ) -> Result<RegionSetSubtree, String>;

    async fn fetch_subtree(&self, id: String) -> Result<RegionSetSubtree, String>;

    // Child entity CRUD
}
