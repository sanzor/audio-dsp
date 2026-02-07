use player::sink::AudioSink;

use domain::db::TrackId;

pub struct UserAttachSink {
    pub track_id: TrackId,
    pub sink: Box<dyn AudioSink + Send + Sync>,
}
#[derive(Debug)]
pub struct UserAttachSinkResult {
    pub track_id: TrackId,
    pub sink_id: String,
}
