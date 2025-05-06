use audiolib::audio_buffer::AudioBuffer;

pub struct Track{
    pub info:TrackInfo,
    pub data:AudioBuffer
}
#[derive(Clone)]
pub struct TrackInfo{
    pub name:String
}