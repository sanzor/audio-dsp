use domain::{
    db::{
        db_region::{DbRegion, RegionId},
        db_region_set::{DbRegionSet, RegionSetId},
        db_track::TrackId,
    },
    region_set::{
        copy_region_set_params::CopyRegionSetParams,
        create_region_set_params::CreateRegionSetParams,
        edit_region_set_params::EditRegionSetParams,
        region_set_subtree::RegionSetSubtree,
    },
    regions::{
        add_region_params::AddRegionParams,
        copy_region_params::CopyRegionParams,
        delete_region_params::DeleteRegionParams,
        edit_region_params::EditRegionParams,
    },
};

#[async_trait::async_trait]
pub trait RegionSetsProvider: Send + Sync {
    // ── Region set CRUD ───────────────────────────────────────────────────────
    async fn create_region_set(&self, params: CreateRegionSetParams) -> Result<DbRegionSet, String>;
    async fn get_region_set(&self, set_id: &RegionSetId) -> Result<DbRegionSet, String>;
    async fn get_region_sets_for_track(&self, track_id: &TrackId) -> Result<Vec<DbRegionSet>, String>;
    async fn get_region_sets(&self) -> Result<Vec<DbRegionSet>, String>;
    async fn edit_region_set(&self, params: EditRegionSetParams) -> Result<DbRegionSet, String>;
    async fn delete_region_set(&self, set_id: &RegionSetId) -> Result<(), String>;
    async fn copy_region_set(&self, params: CopyRegionSetParams) -> Result<DbRegionSet, String>;

    // ── Region CRUD (actors reach through this provider for both) ─────────────
    async fn get_regions_for_region_set(&self, set_id: &RegionSetId) -> Result<Vec<DbRegion>, String>;
    async fn add_region(&self, params: AddRegionParams) -> Result<DbRegion, String>;
    async fn edit_region(&self, params: EditRegionParams) -> Result<DbRegion, String>;
    async fn delete_region(&self, params: DeleteRegionParams) -> Result<(), String>;
    async fn copy_region(&self, params: CopyRegionParams) -> Result<DbRegion, String>;
    async fn get_region(&self, region_id: &RegionId) -> Result<DbRegion, String>;

    // ── View ──────────────────────────────────────────────────────────────────
    async fn get_region_set_subtree(&self, set_id: &RegionSetId) -> Result<RegionSetSubtree, String>;
}
