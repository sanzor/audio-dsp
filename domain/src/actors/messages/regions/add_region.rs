pub struct AddRegion{
    pub name:String,
    pub track_id:String,
    pub start_time:f32,
    pub end_time:Option<f32>
}

pub struct AddRegionResult{
    pub region_id:String,
    pub track_id:String,
    pub name:String,
}