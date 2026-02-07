use crate::db::TrackId;

pub struct AddSink {
    pub track_id: TrackId,
    pub sink_id: String,
}

pub struct AddSinkResult {}
