use audiolib::audio_buffer::AudioBuffer;

#[derive(Clone)]
pub struct Track{
    pub info:TrackInfo,
    pub data:AudioBuffer
}
#[derive(Clone)]
#[derive(Debug)]
pub struct TrackInfo{
    pub name:String
}
