use domain::regions::{
    add_region_params::AddRegionParams, copy_region_params::CopyRegionParams,
    edit_region_params::EditRegionParams,
};
use dtos::db::{region_db_dto::RegionDbDto, region_subtree::RegionSubtree};

#[async_trait::async_trait]
pub trait RegionsProvider: Send + Sync {
    /// Get a single region by ID (flat DTO, no nested graph)
    async fn get_region(&self, region_id: &str) -> Result<RegionDbDto, String>;

    /// Get all regions belonging to a region set (flat DTOs, no nested graphs)
    async fn get_regions_for_region_set(
        &self,
        region_set_id: &str,
    ) -> Result<Vec<RegionDbDto>, String>;

    /// Create a new region (returns flat DTO of the new region)
    async fn add_region(&self, params: AddRegionParams) -> Result<RegionDbDto, String>;

    /// Update an existing region (returns flat DTO of the updated region)
    async fn edit_region(&self, params: EditRegionParams) -> Result<RegionDbDto, String>;

    /// Delete a region by ID
    async fn delete_region(&self, region_id: &str) -> Result<(), String>;

    /// Copy a region (returns flat DTO of the new region, graph_id will be None initially)
    async fn copy_region(&self, params: CopyRegionParams) -> Result<RegionSubtree, String>;

    async fn fetch_subtree(&self, region_id: &str) -> Result<RegionSubtree, String>;
}
