use audiolib::audio_buffer::AudioBuffer;
use serde::Serialize;

#[derive(Clone)]
#[derive(Serialize)]
pub struct Track{
    pub info:TrackInfo,
    pub data:AudioBuffer
}
#[derive(Clone)]
#[derive(Debug,Serialize)]
pub struct TrackInfo{
    pub name:String
}

pub struct TrackRef<'a>{
    pub inner:&'a Track,
}

pub struct TrackRefMut<'a>{
    pub inner:&'a mut Track,
}
