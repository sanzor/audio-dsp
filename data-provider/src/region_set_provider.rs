use domain::{add_region_params::AddRegionParams, edit_region_params::EditRegionParams, region_set::RegionSet};
#[async_trait::async_trait]
pub trait RegionSetsProvider: Send + Sync {
    // Aggregate-level CRUD
    async fn create_region_set(&self, track_id: &str, name: Option<String>)
        -> Result<RegionSet, String>;

    async fn get_region_set(&self, set_id: &str)
        -> Result<RegionSet, String>;

    async fn update_region_set(&self, set_id: &str, name: Option<String>)
        -> Result<RegionSet, String>;

    async fn delete_region_set(&self, set_id: &str)
        -> Result<(), String>;

    // Child entity CRUD
    async fn add_region(&self, set_id: &str, params: AddRegionParams)
        -> Result<RegionSet, String>;

    async fn edit_region(&self, set_id: &str, params: EditRegionParams)
        -> Result<RegionSet, String>;

    async fn delete_region(&self, set_id: &str, region_id: &str)
        -> Result<RegionSet, String>;
}