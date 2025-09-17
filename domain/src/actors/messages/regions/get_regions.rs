use serde::Serialize;

use crate::regions::region::Region;

pub struct GetRegions{
    pub track_id:String
}
#[derive(Serialize)]
pub struct GetRegionsResult{
    pub track_id:String,
    pub regions:Vec<Region>
}