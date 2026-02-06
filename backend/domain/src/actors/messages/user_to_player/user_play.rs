use serde::Serialize;

#[derive(Serialize)]
pub struct UserPlay {
    pub track_id: String,
}

pub struct UserPlayResult {}
