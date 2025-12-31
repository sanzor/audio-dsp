use uuid::Uuid;

pub struct NewRegionSet {
    // Note the references! The data is borrowed from the owned DTO.
    pub id: Uuid,
    pub track_id: Uuid,
    pub track_length: f64, // f64/f32 are Copy, so reference is optional
    pub name: str,
}
