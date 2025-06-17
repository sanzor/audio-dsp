use dsp_domain::track::Track;
use player::audio_sink::AudioSink;

pub struct AudioPlayerActorParams{
    pub sink: Box<dyn AudioSink+Send +'static>,
    pub cursor: usize,
    pub track: Track,
}