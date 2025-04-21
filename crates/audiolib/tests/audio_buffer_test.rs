use std::vec;
mod common;
use audiolib::{audio_buffer::{AudioBuffer,Channels}, audio_transform::AudioTransform, file_reader::read_audio_file};
use rstest::*;
#[rstest]
#[case(2.0)]
pub fn can_gain(#[case] factor:f32){
    
    let original_gain=1.5_f32;
    let samples:Vec<f32>=vec![original_gain];
    let sample_rate=32.0;
    let channels=Channels::Mono;
    let mut buffer=AudioBuffer{samples:samples,channels:channels,sample_rate:sample_rate};
    let x= buffer.gain(factor);
    assert_eq!(x.samples[0],(original_gain*factor));
}


#[rstest]
#[case("dragons.wav")]
pub fn can_parse_file(#[case]filename:&str){
    let f = common::test_data(filename);
    assert!(f.exists(), "File not found at path: {:?}", f);
    println!("Trying to open file: {:?}", f);
    let file=read_audio_file(f.to_str().unwrap());
    assert!(file.is_ok());
}