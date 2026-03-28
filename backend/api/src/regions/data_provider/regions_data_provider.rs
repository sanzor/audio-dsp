use domain::{
    db::{
        db_region::{DbRegion, RegionId},
        db_region_set::{DbRegionSet, RegionSetId},
    },
    regions::{
        add_region_params::AddRegionParams,
        copy_region_params::CopyRegionParams,
        delete_region_params::DeleteRegionParams,
        edit_region_params::EditRegionParams,
    },
};

#[async_trait::async_trait]
pub trait RegionsDataProvider: Send + Sync {
    async fn get_region(&self, region_id: &RegionId) -> Result<DbRegion, String>;
    async fn get_region_set(&self, set_id: &RegionSetId) -> Result<DbRegionSet, String>;
    async fn get_regions_for_region_set(&self, set_id: &RegionSetId) -> Result<Vec<DbRegion>, String>;
    async fn add_region(&self, params: AddRegionParams) -> Result<DbRegion, String>;
    async fn edit_region(&self, params: EditRegionParams) -> Result<DbRegion, String>;
    async fn delete_region(&self, params: DeleteRegionParams) -> Result<(), String>;
    async fn copy_region(&self, params: CopyRegionParams) -> Result<DbRegion, String>;
}
