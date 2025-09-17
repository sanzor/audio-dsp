use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Region{
    pub region_set_id:String,
    pub region_id:String,
    pub name:String,
    pub start_time:f32,
    pub end_time:f32
}   