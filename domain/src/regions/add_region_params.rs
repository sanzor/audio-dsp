pub struct AddRegionParams{
    pub region_set_id:String,
    pub start_time:f32,
    pub end_time:f32,
    pub name:String
}

pub struct AddRegionResult{
    pub region_set_id:String,
    pub region_id:String,
    pub name:String
}
