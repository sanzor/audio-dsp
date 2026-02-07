use domain::db::TrackId;

pub struct UserRemoveSink {
    pub sink_id: String,
    pub track_id: TrackId,
}

pub struct UserRemoveSinkResult {}
