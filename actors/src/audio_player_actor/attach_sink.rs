use player::audio_sink::AudioSink;

pub struct AttachSink {
    pub sink_id: String,
    pub sink: Box<dyn AudioSink>,
}

pub struct AttachSinkResult {}
