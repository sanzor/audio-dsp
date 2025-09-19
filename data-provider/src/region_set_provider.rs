use std::collections::HashMap;

use domain::regions::{
    add_region_params::AddRegionParams, create_region_set_params::CreateRegionSetParams,
    delete_region_params::DeleteRegionParams, edit_region_params::EditRegionParams,
    edit_region_set_params::EditRegionSetParams, region_set::RegionSet,
};

#[async_trait::async_trait]
pub trait RegionSetsProvider: Send + Sync {
    // Aggregate-level CRUD
    async fn create_region_set(&self, params: CreateRegionSetParams) -> Result<RegionSet, String>;

    async fn get_region_set(&self, set_id: &str) -> Result<RegionSet, String>;

    async fn get_region_sets_for_track(&self, track_id: &str) -> Result<Vec<RegionSet>, String>;

    async fn get_region_sets(&self) -> Result<HashMap<String,Vec<RegionSet>>, String>;

    async fn edit_region_set(&self, params: EditRegionSetParams) -> Result<RegionSet, String>;

    async fn delete_region_set(&self, set_id: &str) -> Result<(), String>;

    // Child entity CRUD
    async fn add_region(&self, params: AddRegionParams) -> Result<RegionSet, String>;

    async fn edit_region(&self, params: EditRegionParams) -> Result<RegionSet, String>;

    async fn delete_region(&self, params: DeleteRegionParams) -> Result<(), String>;
}
