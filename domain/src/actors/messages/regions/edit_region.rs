pub struct EditRegion{
    pub region_id:String,
    pub name:Option<String>,
    pub start_time:Option<f32>,
    pub end_time:Option<f32>
}

pub struct EditRegionResult{
    pub region_id:String,
    pub track_id:String,
    pub name:String,
    pub start_time:f32,
    pub end_time:f32
}