use crate::db::region_set_subtree::RegionSetSubtree;

#[derive(Clone, Debug)]
pub struct TrackSubtree {
    pub id: String,
    pub name: String,
    pub region_sets: Vec<RegionSetSubtree>,
}
