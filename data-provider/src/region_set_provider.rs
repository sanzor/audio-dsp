use std::collections::HashMap;

use domain::{
    region_set::{
        copy_region_set_params::CopyRegionSetParams,
        create_region_set_params::CreateRegionSetParams,
        edit_region_set_params::EditRegionSetParams,
    },
    regions::{
        add_region_params::AddRegionParams, copy_region_params::CopyRegionParams,
        delete_region_params::DeleteRegionParams, edit_region_params::EditRegionParams,
        region_set::RegionSet,
    },
};

#[async_trait::async_trait]
pub trait RegionSetsProvider: Send + Sync {
    // Aggregate-level CRUD
    async fn create_region_set(&self, params: CreateRegionSetParams) -> Result<RegionSet, String>;

    async fn get_region_set(&self, set_id: &str) -> Result<RegionSet, String>;

    async fn get_region_sets_for_track(&self, track_id: &str) -> Result<Vec<RegionSet>, String>;

    async fn get_region_sets(&self) -> Result<HashMap<String, Vec<RegionSet>>, String>;

    async fn edit_region_set(&self, params: EditRegionSetParams) -> Result<RegionSet, String>;

    async fn delete_region_set(&self, set_id: &str) -> Result<(), String>;

    async fn copy_region_set(&self, params: CopyRegionSetParams) -> Result<RegionSet, String>;

    // Child entity CRUD
    async fn add_region(&self, params: AddRegionParams) -> Result<RegionSet, String>;

    async fn edit_region(&self, params: EditRegionParams) -> Result<RegionSet, String>;

    async fn delete_region(&self, params: DeleteRegionParams) -> Result<(), String>;

    async fn copy_region(&self, params: CopyRegionParams) -> Result<RegionSet, String>;
}
