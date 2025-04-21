
mod common;
use audiolib::{audio_buffer::{AudioBuffer,Channels}, audio_transform::AudioTransform};
use audiolib::audio_parse::read_wav_file;
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
#[case("dragons.wav",1.5)]
pub fn can_parse_and_gain(#[case]filename:&str,#[case]gain:f32){
    let path=common::test_data(filename);
    let final_path=path.to_str().unwrap();
    let file=
        read_wav_file(final_path)
        .map(|wav| {
            let mut mut_wav=wav;
            mut_wav.gain(gain)
        });
      
    assert!(file.is_ok());
}