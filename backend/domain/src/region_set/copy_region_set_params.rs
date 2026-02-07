use crate::db::RegionSetId;

pub struct CopyRegionSetParams {
    pub region_set_id: RegionSetId,
    pub region_set_name: String,
}
