
mod common;
use std::fs;
use audiolib::{audio_parse::write_wav_file, audio_transform::AudioTransform};
use audiolib::audio_parse::read_wav_file;
use rstest::*;

#[rstest]
#[case("dragons.wav",1.5)]
pub fn can_parse_and_gain(#[case]filename:&str,#[case]gain:f32){
    let path=common::test_data(filename);
    let wav_stream_with_gain=
        read_wav_file(&path)
        .map(|wav| 
            wav.gain(gain)
        ); 
    assert!(wav_stream_with_gain.is_ok());
}

#[rstest]
#[case("dragons.wav","output.wav",20.0)]
pub fn can_write_wav_file(#[case]input_file:&str,#[case]output_file:&str,#[case]gain:f32){

    let output_path = common::test_data(output_file);

    if output_path.exists() {
        fs::remove_file(&output_path).expect("Failed to remove existing file");
    }

    let path=common::test_data(input_file);
    let wav_stream_with_gain=
         read_wav_file(&path)
        .map(|wav| 
            wav.gain(gain)
        ).unwrap();
    let _= write_wav_file(&wav_stream_with_gain, &output_path);
}