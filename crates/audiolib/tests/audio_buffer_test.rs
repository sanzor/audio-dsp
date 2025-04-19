use std::vec;

use audiolib::audio_buffer::{AudioBuffer,AudioTransform,Channels};

#[test]
pub fn can_gain(){
    let samples:Vec<f32>=vec![1.0,2.0,3.0];
    let sample_rate=32.0;
    let channels=Channels::Mono;
    let buffer=AudioBuffer{samples:samples,channels:channels,sample_rate:sample_rate};
}
