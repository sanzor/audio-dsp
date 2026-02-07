use crate::db::RegionSetId;
use crate::regions::region_set::RegionSet;

pub struct AddRegion {
    pub name: String,
    pub region_set_id: RegionSetId,
    pub start_time: f32,
    pub end_time_policy: EndTimePolicy,
}

pub struct AddRegionResult {
    pub region_set: RegionSet,
}

pub enum EndTimePolicy {
    NextRegionOrEnd,
    FixedLength(f32),
    Explicit(f32),
}
