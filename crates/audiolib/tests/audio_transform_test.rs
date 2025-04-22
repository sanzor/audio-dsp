use rstest::rstest;

#[rstest]
#[case(2.0)]
pub fn can_gain(#[case] factor:f32){
    use audiolib::{audio_buffer::{AudioBuffer, Channels}, audio_transform::AudioTransform};

    
    let original_gain=1.5_f32;
    let samples:Vec<f32>=vec![original_gain];
    let sample_rate=32.0;
    let channels=Channels::Mono;
    let buffer=AudioBuffer{samples:samples,channels:channels,sample_rate:sample_rate};
    let x= buffer.gain(factor);
    assert_eq!(x.samples[0],(original_gain*factor));
}