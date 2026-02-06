use player::sink::AudioSink;

pub struct AttachSink {
    pub sink: Box<dyn AudioSink + Send + Sync>,
}

pub struct AttachSinkResult {
    pub sink_id: String,
}
