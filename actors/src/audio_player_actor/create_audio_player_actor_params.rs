use domain::track::Track;
use player::audio_sink::AudioSink;

pub struct CreateAudioPlayerActorParams {
    pub sink: Box<dyn AudioSink + Sync + Send + 'static>,
    pub cursor: usize,
    pub track: Track,
}
