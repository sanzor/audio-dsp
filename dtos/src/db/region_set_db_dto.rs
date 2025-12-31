use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct RegionSetDbDto {
    pub id: String,
    pub track_id: String,
    pub track_length: f32,
    pub name: String,
}
