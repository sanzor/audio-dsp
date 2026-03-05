use domain::{
    db::{
        db_region_set::{DbRegionSet, RegionSetId},
        db_track::TrackId,
    },
    region_set::{
        copy_region_set_params::CopyRegionSetParams,
        create_region_set_params::CreateRegionSetParams,
        edit_region_set_params::EditRegionSetParams,
    },
};

#[async_trait::async_trait]
pub trait RegionSetsDataProvider: Send + Sync {
    async fn create_region_set(&self, params: CreateRegionSetParams) -> Result<DbRegionSet, String>;
    async fn get_region_set(&self, set_id: &RegionSetId) -> Result<DbRegionSet, String>;
    async fn get_region_sets_for_track(&self, track_id: &TrackId) -> Result<Vec<DbRegionSet>, String>;
    async fn get_region_sets(&self) -> Result<Vec<DbRegionSet>, String>;
    async fn edit_region_set(&self, params: EditRegionSetParams) -> Result<DbRegionSet, String>;
    async fn delete_region_set(&self, set_id: &RegionSetId) -> Result<(), String>;
    async fn copy_region_set(&self, params: CopyRegionSetParams) -> Result<DbRegionSet, String>;
}
