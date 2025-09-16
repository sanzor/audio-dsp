use crate::region::Region;

pub struct GetRegions{
    pub track_id:String
}

pub struct GetRegionsResult{
    pub track_id:String,
    pub regions:Vec<Region>
}