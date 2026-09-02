use std::collections::HashMap;

use domain::{
    db::{
        db_region_set::{DbRegionSet, RegionSetId},
        db_track::TrackId,
    },
    region_set::region_set_subtree::RegionSetSubtree,
};

pub struct CreateRegionSetParams {
    pub track_id: TrackId,
    pub name: Option<String>,
}

pub struct EditRegionSetParams {
    pub region_set_id: RegionSetId,
    pub track_id: TrackId,
    pub name: Option<String>,
}

pub struct CopyRegionSetParams {
    pub region_set_id: RegionSetId,
    pub copy_name: String,
}

#[async_trait::async_trait]
pub trait RegionSetsProvider: Send + Sync {
    async fn create_region_set(&self, params: CreateRegionSetParams)
        -> Result<DbRegionSet, String>;
    async fn get_region_set(&self, set_id: &RegionSetId) -> Result<DbRegionSet, String>;
    async fn get_region_sets(&self) -> Result<HashMap<TrackId, Vec<DbRegionSet>>, String>;
    async fn get_region_sets_for_track(
        &self,
        track_id: &TrackId,
    ) -> Result<Vec<DbRegionSet>, String>;
    async fn edit_region_set(&self, params: EditRegionSetParams) -> Result<DbRegionSet, String>;
    async fn delete_region_set(&self, set_id: &RegionSetId) -> Result<(), String>;
    async fn copy_region_set(
        &self,
        params: CopyRegionSetParams,
    ) -> Result<RegionSetSubtree, String>;
}
