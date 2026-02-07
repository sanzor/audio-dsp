use serde::Serialize;

use crate::db::TrackId;

#[derive(Serialize)]
pub struct UserPlay {
    pub track_id: TrackId,
}

pub struct UserPlayResult {}
