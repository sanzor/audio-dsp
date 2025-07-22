use player::audio_sink::AudioSink;

pub struct UserAttachSink {
    pub track_id: String,
    pub sink: Box<dyn AudioSink + Send + Sync>,
}
#[derive(Debug)]
pub struct UserAttachSinkResult {
    pub track_id: String,
    pub sink_id: String,
}
