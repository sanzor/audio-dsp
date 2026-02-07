use crate::db::TrackId;

pub struct UserSeek {
    pub track_id: TrackId,
    pub position: u32,
}

pub struct UserSeekResult {}
