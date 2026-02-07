use crate::db::TrackId;

pub struct CreateRegionSetParams {
    pub track_id: TrackId,
    pub track_length: f32,
    pub name: Option<String>,
}
