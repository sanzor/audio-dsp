use domain::db::{
    db_region::RegionId,
    db_region_set::RegionSetId,
    db_track::TrackId,
};
use domain::{db::DbRegion, region_set::region_set_subtree::RegionSetSubtree};

pub enum EndTimePolicy {
    NextRegionOrEnd,
    FixedLength(f32),
    Explicit(f32),
}

pub struct AddRegionParams {
    pub name: String,
    pub region_set_id: RegionSetId,
    pub start_time: f32,
    pub end_time_policy: EndTimePolicy,
}

pub struct EditRegionParams {
    pub region_id: RegionId,
    pub name: Option<String>,
    pub start_time: Option<f32>,
    pub end_time: Option<f32>,
}

pub struct DeleteRegionParams {
    pub region_id: RegionId,
}

pub struct CopyRegionParams {
    pub source_region_id: RegionId,
    pub source_region_set_id: RegionSetId,
    pub source_track_id: TrackId,
    pub destination_region_set_id: RegionSetId,
    pub destination_track_id: TrackId,
    pub copy_name: String,
}

#[async_trait::async_trait]
pub trait RegionsProvider: Send + Sync {
    async fn add_region(&self, params: AddRegionParams) -> Result<RegionSetSubtree, String>;
    async fn edit_region(&self, params: EditRegionParams) -> Result<DbRegion, String>;
    async fn delete_region(&self, params: DeleteRegionParams) -> Result<RegionSetSubtree, String>;
    async fn copy_region(&self, params: CopyRegionParams) -> Result<DbRegion, String>;
}
